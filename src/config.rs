pub enum After {
  Refresh,
  Exit,
}

pub struct KeyBinding {
  pub key: char,
  pub command: String,
  pub confirm: bool,
  pub after: After,
}

pub struct Profile {
  pub name: String,
  pub registered_commands: Vec<String>,
  pub key_bindings: Vec<KeyBinding>,
  pub lines_to_skip: usize,
}

pub struct Config {
  pub profiles: Vec<Profile>,
}

impl Config {
  pub fn new() -> Config {
    // just doing a dummy one for now
    Config {
      profiles: vec![
        Profile {
          name: String::from("ls"),
          registered_commands: vec![String::from("ls -1"), String::from("ls -a")],
          key_bindings: vec![
            KeyBinding {
              key: 'd',
              command: String::from("rm $0"),
              confirm: true,
              after: After::Refresh,
            },
            KeyBinding {
              key: 'o',
              command: String::from("code -r $0"),
              confirm: true,
              after: After::Refresh,
            },
            KeyBinding {
              key: 'u',
              command: String::from("cd $0"),
              confirm: true,
              after: After::Refresh,
            },
          ],
          lines_to_skip: 0,
        },
        Profile {
          name: String::from("git status"),
          registered_commands: vec![String::from("git status --short")],
          key_bindings: vec![
            KeyBinding {
              key: 'A',
              command: String::from("git add $0"),
              confirm: true,
              after: After::Refresh,
            },
            KeyBinding {
              key: 'a',
              command: String::from("git reset $1"),
              confirm: true,
              after: After::Refresh,
            },
            KeyBinding {
              key: 'd',
              command: String::from("rm $1"),
              confirm: true,
              after: After::Refresh,
            },
          ],
          lines_to_skip: 0,
        },
        Profile {
          name: String::from("docker ps"),
          registered_commands: vec![String::from("docker ps")],
          key_bindings: vec![
            KeyBinding {
              key: 's',
              command: String::from("docker stop $0"),
              confirm: true,
              after: After::Refresh,
            },
            KeyBinding {
              key: 'r',
              command: String::from("docker restart $0"),
              confirm: true,
              after: After::Refresh,
            },
            KeyBinding {
              key: 'd',
              command: String::from("docker kill $0"),
              confirm: true,
              after: After::Refresh,
            },
          ],
          lines_to_skip: 0,
        },
      ],
    }
  }
}
