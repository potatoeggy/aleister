struct GetStartingHandStage {
    player1_ok: bool,
    player2_ok: bool,
}

impl GetStartingHandStage {
    fn new() -> GetStartingHandStage {
        GetStartingHandStage {
            player1_ok: false,
            player2_ok: false,
        }
    }
}

struct RollPhase;
struct ActionPhase;
struct BattlePhase;
struct EndPhase;

struct Game<S: GameStage> {
    state: Box<GameState>,
    stage: S,
}

trait GameStage {}
impl GameStage for GetStartingHandStage {}
impl GameStage for RollPhase {}
impl GameStage for ActionPhase {}
impl GameStage for BattlePhase {}
impl GameStage for EndPhase {}

impl Game<GetStartingHandStage> {
    fn new() -> Game<GetStartingHandStage> {
        let mut state = GameState::new();
        state.player1.hand = state.player1.deck.drain(0..5).collect();
        state.player2.hand = state.player2.deck.dJrain(0..5).collect();
        Game {
            state: Box::new(state),
            stage: GetStartingHandStage::new(),
        }
    }

    fn redraw_hand(self, player: Player, indices: Vec<usize>) -> () {
        let mut state = *self.state;
        let player = match player {
            Player::Player1 => &mut state.player1,
            Player::Player2 => &mut state.player2,
        };
        let mut new_hand = vec![];
        for i in indices {
            new_hand.push(player.hand.remove(i));
        }
    }
}

impl Game<RollPhase> {
    fn roll_phase(self) -> Game<RollPhase> {
        Game {
            state: self.state,
            stage: RollPhase,
        }
    }
}

struct TurnPlayer1;
struct TurnPlayer2;

enum Player {
    Player1,
    Player2,
}
struct GameState {
    player1: PlayerState,
    player2: PlayerState,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            player1: PlayerState::new(),
            player2: PlayerState::new(),
        }
    }
}

enum FieldCharacterSelect {
    First,
    Second,
    Third,
}
type FieldCharacters = (CharacterCard, CharacterCard, CharacterCard);
struct PlayerState {
    hand: Vec<ActionCard>,
    deck: Vec<ActionCard>,
    field: FieldCharacters,
    active_char: FieldCharacterSelect,
}

impl PlayerState {
    fn new() -> PlayerState {
        PlayerState {
            hand: vec![],
            deck: vec![],
            field: (
                CharacterCard {
                    name: "A".to_string(),
                },
                CharacterCard {
                    name: "B".to_string(),
                },
                CharacterCard {
                    name: "C".to_string(),
                },
            ),
            active_char: FieldCharacterSelect::First,
        }
    }
}

struct Card {
    name: String,
}

struct CharacterCard {
    name: String,
}

struct ActionCard {
    name: String,
}

fn main() {
    let game = Game::new();
    let game2 = game.get_starting_hand();
    println!("Hello, world!");
}
