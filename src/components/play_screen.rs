use crate::server::{decrypt_game_data, get_game_data, get_key};
use leptos::{prelude::*, task::spawn_local};

const SCALE: usize = 15;
const TICKS_PER_FRAME: usize = 20;

#[component]
pub fn GameScreen(game_to_play: RwSignal<String>) -> impl IntoView {
    let status = RwSignal::new(String::new());

    #[cfg(feature = "hydrate")]
    {
        use std::cell::{Cell, RefCell};
        use std::rc::Rc;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use wasm_bindgen::prelude::*;

        let emu: Rc<RefCell<Option<crate::vm::EmuWasm>>> = Rc::new(RefCell::new(None));

        let running = Arc::new(AtomicBool::new(true));
        let running_cleanup = running.clone();
        on_cleanup(move || {
            running_cleanup.store(false, Ordering::Relaxed);
        });

        // Keyboard listeners (no-op when emu is None)
        let document = web_sys::window().unwrap().document().unwrap();

        let emu_kd = emu.clone();
        let keydown_cb = Closure::wrap(Box::new(move |evt: web_sys::KeyboardEvent| {
            if let Some(e) = emu_kd.borrow_mut().as_mut() {
                e.keypress(evt, true);
            }
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("keydown", keydown_cb.as_ref().unchecked_ref())
            .unwrap();
        keydown_cb.forget();

        let emu_ku = emu.clone();
        let keyup_cb = Closure::wrap(Box::new(move |evt: web_sys::KeyboardEvent| {
            if let Some(e) = emu_ku.borrow_mut().as_mut() {
                e.keypress(evt, false);
            }
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("keyup", keyup_cb.as_ref().unchecked_ref())
            .unwrap();
        keyup_cb.forget();

        // Game loop (no-op when emu is None)
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        let emu_loop = emu.clone();
        let running_loop = running.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if !running_loop.load(Ordering::Relaxed) {
                return;
            }
            if let Some(e) = emu_loop.borrow_mut().as_mut() {
                for _ in 0..TICKS_PER_FRAME {
                    e.tick();
                }
                e.tick_timers();
                e.draw_screen(SCALE);
            }
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>));

        web_sys::window()
            .unwrap()
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();

        // Fetch ROM when game selection changes
        let emu_effect = emu.clone();
        let version = Rc::new(Cell::new(0u32));

        Effect::new(move || {
            let value = game_to_play.get();
            if value.is_empty() {
                return;
            }

            let v = version.get() + 1;
            version.set(v);
            let version_check = version.clone();
            let emu_clone = emu_effect.clone();

            spawn_local(async move {
                if let Some((developer, name)) = value.split_once('|') {
                    use crate::wallet::{get_public_key, sign_message, Message};
                    use base64::{engine::general_purpose::STANDARD, Engine};

                    // 1. Sign message
                    status.set("Signing...".into());
                    let valid_period = (js_sys::Date::now() / 1000.0) as i64 + 7200;
                    let (signature, _) = match sign_message(Message {
                        valid_period,
                        game_name: name.to_string(),
                    })
                    .await
                    {
                        Ok(r) => r,
                        Err(e) => {
                            status.set(format!("Error: {e}"));
                            return;
                        }
                    };

                    // 2. Get decryption key
                    status.set("Getting decryption key...".into());
                    let player = get_public_key().await;
                    let key_response = match get_key(
                        name.to_string(),
                        developer.to_string(),
                        player,
                        signature,
                        valid_period,
                    )
                    .await
                    {
                        Ok(r) => r,
                        Err(e) => {
                            status.set(format!("Error: {e}"));
                            return;
                        }
                    };

                    // 3. Fetch encrypted game data from chain
                    status.set("Fetching game data...".into());
                    let encrypted_bytes =
                        match get_game_data(developer.to_string(), name.to_string()).await {
                            Ok(r) => r,
                            Err(e) => {
                                status.set(format!("Error: {e}"));
                                return;
                            }
                        };

                    // 4. Decrypt server-side
                    status.set("Decrypting...".into());
                    let decrypted_b64 = match decrypt_game_data(
                        key_response.encryption_key,
                        key_response.nonce,
                        STANDARD.encode(&encrypted_bytes),
                    )
                    .await
                    {
                        Ok(r) => r,
                        Err(e) => {
                            status.set(format!("Error: {e}"));
                            return;
                        }
                    };
                    let rom_bytes = match STANDARD.decode(&decrypted_b64) {
                        Ok(r) => r,
                        Err(e) => {
                            status.set(format!("Error: {e}"));
                            return;
                        }
                    };

                    // 5. Load into emulator
                    if version_check.get() != v {
                        return;
                    }
                    let data = js_sys::Uint8Array::from(rom_bytes.as_slice());
                    let mut emu_ref = emu_clone.borrow_mut();
                    if emu_ref.is_none() {
                        *emu_ref = Some(crate::vm::EmuWasm::new());
                    }
                    let e = emu_ref.as_mut().unwrap();
                    e.reset();
                    e.load_game(data);
                    status.set("Game loaded!".into());
                }
            });
        });
    }

    view! {
        <p>{move || status.get()}</p>
        <canvas
            id="canvas"
            width="960"
            height="480"
            style="border: 1px solid white; background: black;"
        />
    }
}
