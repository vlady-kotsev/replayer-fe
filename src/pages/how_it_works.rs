use leptos::prelude::*;

#[component]
pub fn HowItWorksPage() -> impl IntoView {
    view! {
        <div class="how-it-works">
            <h1>"How It Works"</h1>

            <div class="hiw-section hiw-publish">
                <h2>"1. Publish a Game"</h2>
                <ol>
                    <li>"Connect your Phantom wallet"</li>
                    <li>"Register as a developer"</li>
                    <li>"Upload your CHIP-8 game ROM and cover image"</li>
                    <li>"Game data is encrypted and stored on the Solana blockchain"</li>
                    <li>"Set your price and max supply — your game is live"</li>
                </ol>
            </div>

            <div class="hiw-section hiw-buy">
                <h2>"2. Buy a Game"</h2>
                <ol>
                    <li>"Browse available games in the marketplace"</li>
                    <li>"Click Buy and confirm the transaction in Phantom"</li>
                    <li>"An NFT is minted to your wallet as proof of ownership"</li>
                    <li>"Revenue is split between the developer and the platform"</li>
                </ol>
            </div>

            <div class="hiw-section hiw-play">
                <h2>"3. Play a Game"</h2>
                <ol>
                    <li>"Select an owned game from your library"</li>
                    <li>"Sign an authentication message to verify ownership"</li>
                    <li>"The game is fetched from the blockchain and decrypted"</li>
                    <li>"Play directly in your browser via the built-in CHIP-8 emulator"</li>
                </ol>
            </div>
        </div>
    }
}
