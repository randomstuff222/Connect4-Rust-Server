use super::board_me;

#[derive(Debug)]
pub enum Move {
    LEFT,
    RIGHT,
    DROP,
}

pub trait GameInterface {
    fn name(&self) -> String;

    fn play(
        &self,
        board: &board_me::Board,
        player_color: usize,
    ) -> u8;
}

pub type InterfaceObject = dyn GameInterface;
