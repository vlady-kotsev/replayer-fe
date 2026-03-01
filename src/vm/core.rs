use rand::random;

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Emulator {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_registers: [u8; NUM_REGISTERS],
    i_register: u16,
    stack: [u16; STACK_SIZE],
    sp: u16,
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; NUM_KEYS],
}

const STARTING_ADDRESS: u16 = 0x200;

impl Emulator {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: STARTING_ADDRESS,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_registers: [0; NUM_REGISTERS],
            i_register: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; NUM_KEYS],
        };
        new_emu.ram[0..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    pub fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn push_stack(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn reset(&mut self) {
        self.pc = STARTING_ADDRESS;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
        self.v_registers = [0; NUM_REGISTERS];
        self.i_register = 0;
        self.stack = [0; STACK_SIZE];
        self.sp = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keys = [false; NUM_KEYS];
        self.ram[0..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        let op = self.fetch();

        self.execute(op);
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return, // Nop
            (0, 0, 0xE, 0) => {
                // Clear_screen
                self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
            }
            (0, 0, 0xE, 0xE) => {
                // Return from subroutine
                let ret_addr = self.pop_stack();
                self.pc = ret_addr;
            }
            (1, _, _, _) => {
                // Jump to
                let destination_address = op & 0x0FFF;
                self.pc = destination_address;
            }
            (2, _, _, _) => {
                // Call subroutine
                let subroutine_address = op & 0x0FFF;
                self.push_stack(self.pc);
                self.pc = subroutine_address;
            }
            (3, _, _, _) => {
                // Skip next if VX == NN
                let register_num = digit2 as usize;
                let register = self.v_registers[register_num];
                let num_to_compare = (op & 0x00FF) as u8;

                if register == num_to_compare {
                    self.pc += 2; // each instructon is 2 bytes
                }
            }
            (4, _, _, _) => {
                // Skip next if VX != NN
                let register_num = digit2 as usize;
                let register = self.v_registers[register_num];
                let num_to_compare = (op & 0x00FF) as u8;

                if register != num_to_compare {
                    self.pc += 2; // each instructon is 2 bytes
                }
            }
            (5, _, _, 0) => {
                // Skip next if VX == VY
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                if self.v_registers[register_num_x] == self.v_registers[register_num_y] {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => {
                // Save value into register
                let register_num = digit2 as usize;
                let value = (op & 0x00FF) as u8;
                self.v_registers[register_num] = value;
            }
            (7, _, _, _) => {
                // Add value to register
                let register_num = digit2 as usize;
                let value = (op & 0x00FF) as u8;
                self.v_registers[register_num] = self.v_registers[register_num].wrapping_add(value);
            }
            (8, _, _, 0) => {
                // Save register y in register x
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                self.v_registers[register_num_x] = self.v_registers[register_num_y];
            }
            (8, _, _, 1) => {
                // Bitwise OR
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                self.v_registers[register_num_x] |= self.v_registers[register_num_y];
            }
            (8, _, _, 2) => {
                // Bitwise AND
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                self.v_registers[register_num_x] &= self.v_registers[register_num_y];
            }
            (8, _, _, 3) => {
                // Bitwise XOR
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                self.v_registers[register_num_x] ^= self.v_registers[register_num_y];
            }
            (8, _, _, 4) => {
                // Sum
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                let (result, has_overflow) = self.v_registers[register_num_x]
                    .overflowing_add(self.v_registers[register_num_y]);

                self.v_registers[0xF] = if has_overflow { 1 } else { 0 };
                self.v_registers[register_num_x] = result;
            }
            (8, _, _, 5) => {
                // Substract
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                let (result, has_underflow) = self.v_registers[register_num_x]
                    .overflowing_sub(self.v_registers[register_num_y]);

                self.v_registers[0xF] = if has_underflow { 0 } else { 1 };
                self.v_registers[register_num_x] = result;
            }
            (8, _, _, 6) => {
                // Right shift by one
                let register_number = digit2 as usize;
                let last_bit = self.v_registers[register_number] & 1;
                self.v_registers[register_number] >>= 1;
                self.v_registers[0xF] = last_bit;
            }
            (8, _, _, 7) => {
                // Substract register x from register y and save in x
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                let (result, has_underflow) = self.v_registers[register_num_y]
                    .overflowing_sub(self.v_registers[register_num_x]);

                self.v_registers[0xF] = if has_underflow { 0 } else { 1 };
                self.v_registers[register_num_x] = result;
            }
            (8, _, _, 0xE) => {
                // Left shift by one
                let register_num = digit2 as usize;
                let first_bit = (self.v_registers[register_num] >> 7) & 1;

                self.v_registers[0xF] = first_bit;
                self.v_registers[register_num] <<= 1;
            }
            (9, _, _, 0) => {
                // Skip next if VX != VY
                let register_num_x = digit2 as usize;
                let register_num_y = digit3 as usize;

                if self.v_registers[register_num_x] != self.v_registers[register_num_y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                // Set I_register to value
                let value = op & 0x0FFF;
                self.i_register = value;
            }
            (0xB, _, _, _) => {
                // jump to V0 + value
                let value = op & 0x0FFF;
                let v0 = self.v_registers[0] as u16;
                self.pc = v0 + value;
            }
            (0xC, _, _, _) => {
                // Get random num, then & with value, then save in register
                let register_num = digit2 as usize;
                let value = (op & 0x00FF) as u8;
                let rng: u8 = random();
                self.v_registers[register_num] = rng & value;
            }
            (0xD, _, _, _) => {
                // Draw
                let draw_x = self.v_registers[digit2 as usize] as u16;
                let draw_y = self.v_registers[digit3 as usize] as u16;
                let num_rows = digit4;

                let mut flipped = false;

                for row in 0..num_rows {
                    let sprite_address = self.i_register + row;
                    let pixels = self.ram[sprite_address as usize];

                    for col in 0..8 {
                        // get current bit of sprite in ram
                        let current_bit = (pixels >> (7 - col)) & 1;

                        if current_bit != 0 {
                            // screen draw location of current bit
                            let x = (draw_x + col) as usize % SCREEN_WIDTH;
                            let y = (draw_y + row) as usize % SCREEN_HEIGHT;
                            // get position in draw array
                            let screen_index = x + SCREEN_WIDTH * y;
                            // flipped is used for collision detection (checks if any bit was true before modification)
                            flipped |= self.screen[screen_index];

                            // toggle
                            self.screen[screen_index] ^= true;
                        }
                    }
                }
                if flipped {
                    self.v_registers[0xF] = 1;
                } else {
                    self.v_registers[0xF] = 0;
                }
            }
            (0xE, _, 9, 0xE) => {
                // Skip if Key Pressed
                let register_num = digit2 as usize;
                let key_num = self.v_registers[register_num] as usize;

                let is_pressed = self.keys[key_num];
                if is_pressed {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => {
                // Skip if Key Not Pressed
                let register_num = digit2 as usize;
                let key_num = self.v_registers[register_num] as usize;

                let is_pressed = self.keys[key_num];
                if !is_pressed {
                    self.pc += 2;
                }
            }
            (0xF, _, 0, 7) => {
                // save delay_time in register
                let register_num = digit2 as usize;
                self.v_registers[register_num] = self.delay_timer;
            }
            (0xF, _, 0, 0xA) => {
                // wait for key press, and when is pressed save key index in register
                let register_num = digit2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_registers[register_num] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.pc -= 2;
                }
            }
            (0xF, _, 1, 5) => {
                // save register in delay_time
                let register_num = digit2 as usize;
                self.delay_timer = self.v_registers[register_num];
            }
            (0xF, _, 1, 8) => {
                // save register in sound_timer
                let register_num = digit2 as usize;
                self.sound_timer = self.v_registers[register_num];
            }
            (0xF, _, 1, 0xE) => {
                // i_register += value in register
                let register_num = digit2 as usize;
                self.i_register = self
                    .i_register
                    .wrapping_add(self.v_registers[register_num] as u16);
            }
            (0xF, _, 2, 9) => {
                // set I_register to the sprite corresponding to the value in register
                let register_num = digit2 as usize;
                let value = self.v_registers[register_num] as u16;
                let value_address = 5 * value; // each sprite is 5 bytes and start from 0 in ram
                self.i_register = value_address;
            }
            (0xF, _, 3, 3) => {
                // Binary-Coded Decimal
                let register_num = digit2 as usize;
                let num = self.v_registers[register_num] as f32;

                let hundreds = (num / 100.0).floor() as u8;
                let tens = ((num / 10.0) % 10.0).floor() as u8;
                let ones = (num % 10.0) as u8;

                self.ram[self.i_register as usize] = hundreds;
                self.ram[(self.i_register + 1) as usize] = tens;
                self.ram[(self.i_register + 2) as usize] = ones;
            }
            (0xF, _, 5, 5) => {
                // store values of register from 0 to register_num inclusive into ram
                let register_num = digit2 as usize;
                let i_register = self.i_register as usize;
                for i in 0..=register_num {
                    self.ram[i_register + i] = self.v_registers[i];
                }
            }
            (0xF, _, 6, 5) => {
                // store values into register from 0 to register_num inclusive from ram
                let register_num = digit2 as usize;
                let i_register = self.i_register as usize;
                for i in 0..=register_num {
                    self.v_registers[i] = self.ram[i_register + i];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // make sound
            }
            self.sound_timer -= 1;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypressed(&mut self, index: usize, pressed: bool) {
        self.keys[index] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = STARTING_ADDRESS as usize;
        let end = start + data.len();

        self.ram[start..end].copy_from_slice(data);
    }
}

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
