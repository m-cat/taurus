use game::Game;
use dungeon::Dungeon;
use actor::ActResult;
use actor::Actor;
use console::GameConsole;

pub fn player_act(player: &mut Actor, game: &Game, dungeon: &mut Dungeon, console: &mut GameConsole) -> ActResult {
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
