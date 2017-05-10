use game::Game;
use dungeon::Dungeon;
use actor::ActResult;
use actor::Actor;

pub fn player_act(player: &mut Actor, game: &Game, dungeon: &mut Dungeon) -> ActResult {
    // Check if the game window was closed
    let mut console = game.console.borrow_mut();
    if console.window_closed() {
        return ActResult::WindowClosed;
    } // TODO: how does window_closed work?

    // Draw the game to the screen
    // TODO
    console.put_char(1, 1, '@');
    console.wait_for_keypress(true);

    // TODO
    // while user input is not a game action
    // Wait for user input

    // Respond to user input

    ActResult::None
}
