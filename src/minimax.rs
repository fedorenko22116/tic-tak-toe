#[derive(Clone, Debug, PartialEq)]
pub enum Player {
    Human = -1,
    Blank = 0,
    Computer = 1,
}

impl Default for Player {
    fn default() -> Self {
        Player::Blank
    }
}

impl Player {
    pub fn is(&self, player: &Player) -> bool {
        self == player
    }

    pub fn get_opponent(&self) -> Self {
        match self {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
            _ => Player::Blank,
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub score: isize,
    pub pos: isize,
}

impl Move {
    fn new(score: isize, pos: isize) -> Self {
        Move { score, pos }
    }
}

pub type Board = Vec<Player>;

pub trait Guesser {
    fn guess(&self, player: &Player, winning: fn(board: &Self, player: &Player) -> bool) -> Move;
    fn is_finished(&self) -> bool;
}

impl Guesser for Board {
    fn guess(&self, player: &Player, winning: fn(board: &Self, player: &Player) -> bool) -> Move {
        if winning(&self, &Player::Computer) {
            return Move::new(Player::Computer as isize * (player.to_owned() as isize), -1);
        } else if winning(&self, &Player::Human) {
            return Move::new(Player::Human as isize * (player.to_owned() as isize), -1);
        }

        let mut cmove = Move::new(-2, -1);

        for (i, p) in self.iter().enumerate() {
            if p.is(&Player::Blank) {
                let mut new_board = self.to_owned();
                new_board[i] = player.to_owned();
                let score = -new_board.guess(&player.get_opponent(), winning).score;

                if cmove.score < score {
                    cmove = Move::new(score, i as isize);
                }
            }
        }

        if cmove.pos == -1 {
            return Move::new(0, 0);
        }

        cmove
    }

    fn is_finished(&self) -> bool {
        !self.contains(&Player::Blank)
    }
}
