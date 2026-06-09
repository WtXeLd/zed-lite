use super::{Client, proto};
use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{Context, EventEmitter, SharedString, SharedUri, Task};
use postage::watch;
use std::sync::{Arc, Weak};

pub type LegacyUserId = u64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ProjectId(pub u64);

impl ProjectId {
    pub fn to_proto(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParticipantIndex(pub u32);

#[derive(Default, Debug)]
pub struct User {
    pub legacy_id: LegacyUserId,
    pub github_login: SharedString,
    pub avatar_uri: SharedUri,
    pub name: Option<String>,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.github_login.cmp(&other.github_login)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.legacy_id == other.legacy_id && self.github_login == other.github_login
    }
}

impl Eq for User {}

pub struct UserStore {
    users: HashMap<u64, Arc<User>>,
    by_github_login: HashMap<SharedString, u64>,
    participant_indices: HashMap<u64, ParticipantIndex>,
    current_user: watch::Receiver<Option<Arc<User>>>,
    _client: Weak<Client>,
}

pub enum Event {
    ParticipantIndicesChanged,
    PrivateUserInfoUpdated,
}

impl EventEmitter<Event> for UserStore {}

impl UserStore {
    pub fn new(client: Arc<Client>, _cx: &Context<Self>) -> Self {
        let (_, current_user) = watch::channel();
        Self {
            users: Default::default(),
            by_github_login: Default::default(),
            participant_indices: Default::default(),
            current_user,
            _client: Arc::downgrade(&client),
        }
    }

    #[cfg(feature = "test-support")]
    pub fn clear_cache(&mut self) {
        self.users.clear();
        self.by_github_login.clear();
    }

    pub fn get_users(
        &self,
        user_ids: Vec<u64>,
        _cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        let users = user_ids
            .into_iter()
            .filter_map(|user_id| self.users.get(&user_id).cloned())
            .collect();
        Task::ready(Ok(users))
    }

    pub fn get_cached_user(&self, user_id: u64) -> Option<Arc<User>> {
        self.users.get(&user_id).cloned()
    }

    pub fn get_user_optimistic(&self, user_id: u64, _cx: &Context<Self>) -> Option<Arc<User>> {
        self.get_cached_user(user_id)
    }

    pub fn get_user(&self, user_id: u64, _cx: &Context<Self>) -> Task<Result<Arc<User>>> {
        Task::ready(
            self.users
                .get(&user_id)
                .cloned()
                .with_context(|| format!("user {user_id} not found")),
        )
    }

    pub fn cached_user_by_github_login(&self, github_login: &str) -> Option<Arc<User>> {
        self.by_github_login
            .get(github_login)
            .and_then(|id| self.users.get(id).cloned())
    }

    pub fn current_user(&self) -> Option<Arc<User>> {
        None
    }

    pub fn watch_current_user(&self) -> watch::Receiver<Option<Arc<User>>> {
        self.current_user.clone()
    }

    pub fn insert(&mut self, users: Vec<proto::User>) -> Vec<Arc<User>> {
        let mut inserted = Vec::with_capacity(users.len());
        for user in users {
            let user = User::new(user);
            if let Some(old) = self.users.insert(user.legacy_id, user.clone())
                && old.github_login != user.github_login
            {
                self.by_github_login.remove(&old.github_login);
            }
            self.by_github_login
                .insert(user.github_login.clone(), user.legacy_id);
            inserted.push(user);
        }
        inserted
    }

    pub fn set_participant_indices(
        &mut self,
        participant_indices: HashMap<u64, ParticipantIndex>,
        cx: &mut Context<Self>,
    ) {
        if participant_indices != self.participant_indices {
            self.participant_indices = participant_indices;
            cx.emit(Event::ParticipantIndicesChanged);
        }
    }

    pub fn participant_indices(&self) -> &HashMap<u64, ParticipantIndex> {
        &self.participant_indices
    }

    pub fn participant_names(
        &self,
        user_ids: impl Iterator<Item = u64>,
        _cx: &gpui::App,
    ) -> HashMap<u64, SharedString> {
        user_ids
            .filter_map(|user_id| {
                self.get_cached_user(user_id)
                    .map(|user| (user_id, user.github_login.clone()))
            })
            .collect()
    }
}

impl User {
    fn new(message: proto::User) -> Arc<Self> {
        Arc::new(User {
            legacy_id: message.id,
            github_login: message.github_login.into(),
            avatar_uri: message.avatar_url.into(),
            name: message.name,
        })
    }
}
