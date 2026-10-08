
/*
bit 0: Endian (0 -> 1): 0 for little, 1 for big
bit 1: is_in_queue (0 -> 1)
bit 2 - 3: game mode (0 -> 3)
bit 4-7: game id -> (0 -> 15)

*/


fn main() {
    let mut state: u8 = 0b1011_1111; //binary literal used _ for readability
    println!("is big endian: {}",is_big_endian(state));
    println!("is in queue: {}",is_in_queue(state));
    println!("game mode: {}", get_game_mode(state));
    println!("game id: {}", get_game_id_via_endian(state));
    state = 0b1011_1110; // change the endian bit from 1 to 0, aka from big to little endian
    println!("game id: {}", get_game_id_via_endian(state)); 



}

fn is_big_endian(state: u8) -> bool {
    state & 1 == 1 //no ; so auto returns , using & operator to make sure endian bit is indeed 1 since 1011_1111 & 0000_0001 returns all zeros except the last bit IF the state zero bit is also 1
}

fn is_in_queue(state: u8) -> bool {
    state & 2 == 2 // if equal to 0b10
}

fn get_game_mode(state: u8) -> String {
    let mode = (state >> 2) & 3; //shift two to get it back to its original numbered form from 0-3. Needs the additional & 3 since the shift still had the game_id bits.
    if mode == 0{
        return String::from("ranked");
    } 
    else if mode == 1{
        return String::from("normal");
    }
    else if mode == 2{
        return String::from("practice");
    }
    else {
        return String::from("bots");
    }
}

fn get_game_id_via_endian(state: u8) -> u8 {
    let game_id = state >> 4; // shift to start from 0
    if is_big_endian(state){
        game_id // just return the given id if 
    }
    else {
        game_id.reverse_bits() >> 4 // if the format is little endian read it backwards, still >> 4 so that it goes back to a 0 start.
    }
}

//Tried to make a change to big endian function that would just use .reverse_bits()