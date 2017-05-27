use game::Game;
use dungeon::Dungeon;
use actor::ActResult;
use actor::Actor;
use console::GameConsole;

/// Creates and returns the player actor.
pub fn player_create(game: &Game) -> Actor {
    Actor::new(game, "player")
}

/// Acts out the player's turn.
pub fn player_act(player: &mut Actor,
                  game: &Game,
                  dungeon: &mut Dungeon,
                  console: &mut GameConsole)
                  -> ActResult {
    // while user input is not a game action

    // Check if the window was closed by the user.
    if console.window_closed() {
        return ActResult::WindowClosed;
    } // TODO: how does window_closed work?

    // TODO
    // Draw the game to the screen
    console.put_char(1, 1, '@');
    console.wait_for_keypress(true);

    // Wait for user input

    // Respond to user input

    ActResult::None
}
