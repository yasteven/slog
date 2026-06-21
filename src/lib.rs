#[macro_export]
macro_rules! error
{ ($n:expr, $($args:tt)*) => 
  { { const POSITIVE : u64 = $n;
      if POSITIVE <= 0
      {// This doesn't work  compile_error!("slog::error!(call_number, \"// must be 1 or more \" " );
      }
    
      static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
      let call_number = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
      if (call_number % $n) == 0 
      { log::error!("[{}] {}", call_number, format!($($args)*));
      }
    }
  };
}
#[macro_export]
macro_rules! info
{ ($n:expr, $($args:tt)*) => 
  { { const POSITIVE : u64 = $n;
      if POSITIVE <= 0
      {// This doesn't work  compile_error!("slog::info!(call_number, \"// must be 1 or more \" " );
      }
    
      static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
      let call_number = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
      if (call_number % $n) == 0 
      { log::info!("[{}] {}", call_number, format!($($args)*));
      }
    }
  };
}
#[macro_export]
macro_rules! debug
{ ($n:expr, $($args:tt)*) => 
  { { const POSITIVE : u64 = $n;
      if POSITIVE <= 0
      {// This doesn't work when i had POSITIVE : i32 to try to do thing compile_error!("slog::debug!(call_number, \"// must be 1 or more \" " );
      }
    
      static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
      let call_number = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
      if (call_number % $n) == 0 
      { log::debug!("[{}] {}", call_number, format!($($args)*));
      }
    }
  };
}
#[macro_export]
macro_rules! trace
{ ($n:expr, $($args:tt)*) => 
  { { const POSITIVE : u64 = $n;
      if POSITIVE <= 0
      { // This doesn't work compile_error!("slog::trace!(call_number, \"// must be 1 or more \" " );
      }
    
      static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
      let call_number = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
      if (call_number % $n) == 0 
      { log::trace!("[{}] {}", call_number, format!($($args)*));
      }
    }
  };
}

