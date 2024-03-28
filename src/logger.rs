/// This module provides a simple logging system that can be used to log messages and errors to multiple destinations.
struct LoggerEntity {
    id: String,
    pub log: Box<dyn Fn(&str)>,
    pub error: Box<dyn Fn(&str)>,
}

static mut LOGGER_ENTITIES: Vec<LoggerEntity> = Vec::new();

/// Write a log message to all logger entities.
pub fn write_log(message: &str) {
    for entity in unsafe { LOGGER_ENTITIES.iter() } {
        (entity.log)(message);
    }
}

/// Write an error message to all logger entities.
pub fn write_error(message: &str) {
    for entity in unsafe { LOGGER_ENTITIES.iter() } {
        (entity.error)(message);
    }
}

/// Add a new logger entity to the system.
pub fn add_logger_entity_fn(id: String, log: fn(&str), error: fn(&str)) {
    for entity in unsafe { LOGGER_ENTITIES.iter() } {
        if entity.id == id {
            panic!("INTERNAL: Logger with id '{}' already exists", id);
        }
    }
    let log_closure: Box<dyn Fn(&str)> = Box::new(move |message: &str| {
        log(message);
    });
    let error_closure: Box<dyn Fn(&str)> = Box::new(move |message: &str| {
        error(message);
    });

    unsafe {
        LOGGER_ENTITIES.push(LoggerEntity {
            id,
            log: log_closure,
            error: error_closure,
        });
    }
}

/// Add a new logger entity to the system.
pub fn add_logger_entity_closure(id: String, log: Box<dyn Fn(&str)>, error: Box<dyn Fn(&str)>) {
    for entity in unsafe { LOGGER_ENTITIES.iter() } {
        if entity.id == id {
            panic!("INTERNAL: Logger with id '{}' already exists", id);
        }
    }

    unsafe {
        LOGGER_ENTITIES.push(LoggerEntity { id, log, error });
    }
}

/// Remove a logger entity from the system.
pub fn remove_logger_entity(id: &str) {
    let mut index = None;
    for (i, entity) in unsafe { LOGGER_ENTITIES.iter() }.enumerate() {
        if entity.id == id {
            index = Some(i);
            break;
        }
    }

    if let Some(index) = index {
        unsafe {
            LOGGER_ENTITIES.remove(index);
        }
    }
}

/// Add the default logger entity that writes to stdout and stderr.
pub fn use_stdout() {
    for entity in unsafe { LOGGER_ENTITIES.iter() } {
        if entity.id == "stdout" {
            return;
        }
    }

    add_logger_entity_fn(
        "stdout".to_string(),
        |message| println!("{}", message),
        |message| println!("{}", message),
    );
}
