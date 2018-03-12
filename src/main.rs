extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate fern;
#[macro_use]
extern crate log;

use fern::colors::{Color, ColoredLevelConfig};

const MISPLACE_INGREDIENTS: bool = true;
const TORCH_IT: bool = true;

fn main() {
    if let Ok(_) = setup_logger() {
        debug!("Logging active.");
    }

    if MISPLACE_INGREDIENTS {
        warn!("The ingredients seem to have been misplaced. This might cause problems");
    }

    if TORCH_IT {
        warn!("The chef is a lava monster and has no concept of heat. Keep an eye on him when he's baking.")
    }

    if let Err(e) = get_pizza() {
        error!("{}", e);
    } else {
        info!("Enjoy the pizza");
    }
}

fn get_pizza() -> Result<(), NoPizza> {
    if let Err(e) = make_a_pizza() {
        Err(NoPizza { cause: e })
    } else {
        Ok(())
    }
}

fn make_a_pizza() -> Result<(), MakePizzaError> {
    get_ingredients()?;
    bake_pizza()?;

    Ok(())
}

fn get_ingredients() -> Result<(), MakePizzaError> {
    if MISPLACE_INGREDIENTS {
        Err(MakePizzaError::NoIngredients)
    } else {
        Ok(())
    }
}

fn bake_pizza() -> Result<(), MakePizzaError> {
    if TORCH_IT {
        Err(MakePizzaError::Torched)
    } else {
        Ok(())
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
    .debug(Color::Magenta)
    .info(Color::Blue)
    .warn(Color::Yellow)
    .error(Color::Red);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

#[derive(Fail, Debug)]
enum MakePizzaError {
    #[fail(display = "The pizza ingredients have been misplaced.")] NoIngredients,
    #[fail(display = "The baking temp was too high and the pizza was torched.")] Torched,
}

#[derive(Fail, Debug)]
#[fail(display = "Could not make pizza: {}", cause)]
struct NoPizza {
    cause: MakePizzaError,
}
