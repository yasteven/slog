#[macro_export]
macro_rules! error
{ ($n:expr, $($args:tt)*) => 
  { { static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
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
  { { static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
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
  { { static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
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
  { { static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
      let call_number = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
      if (call_number % $n) == 0 
      { log::trace!("[{}] {}", call_number, format!($($args)*));
      }
    }
  };
}

