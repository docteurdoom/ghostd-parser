use serde::{Deserialize, Serialize};
// Deserialisation requires lifetime 'de to outlive 'static.
// This is why private is converted to public which is deserializable.
#[derive(Debug, Copy, Clone)]
pub struct PrivatePool {
    pub pubkey: &'static str,
    pub url: &'static str,
    pub is_active: bool,
}

impl PrivatePool {
    pub fn getpool(self) -> Pool {
        Pool {
            pubkey: self.pubkey.to_string(),
            url: Some(self.url.to_string()),
            pool_is_active: Some(self.is_active),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub pubkey: String,
    pub url: Option<String>,
    pub pool_is_active: Option<bool>,
}

const MYGHOST: PrivatePool = PrivatePool {
    pubkey: "gcs179wukwy95kewa6pex7f47w3xuzn3nywqdng394",
    url: "https://myghost.org/",
    is_active: true,
};
const MEGA: PrivatePool = PrivatePool {
    pubkey: "gcs1wzjvh9cmdf5h9atk785tvmu68729534rzu34p6",
    url: "https://mega.myghost.org/",
    is_active: true,
};
const GHOSTCSP: PrivatePool = PrivatePool {
    pubkey: "gcs1gvdfylxy4597rq5qutqelr9m37ewtsvzkza8j2",
    url: "https://ghostcsp.ddns.net/",
    is_active: true,
};
const SUPERGHOSTPOS: PrivatePool = PrivatePool {
    pubkey: "gcs1zn850aeltu0d85fruw4wf5yt2e4nj990802p2r",
    url: "https://superghostpos.ru/",
    is_active: true,
};
const POOLGHOSTRUS: PrivatePool = PrivatePool {
    pubkey: "gcs169zkwmr9zt8mz2epql8wnly3dyf4hkavcprrm2",
    url: "https://пул.гост.рус",
    is_active: true,
};

const GHOSTAKE: PrivatePool = PrivatePool {
    pubkey: "gcs1al0tw5g8danpluh2rsqxkd5cj4sc08wc2tsvjt",
    url: "https://ghostake.com",
    is_active: false,
};
const COLDSTAKE_IO: PrivatePool = PrivatePool {
    pubkey: "gcs12ezltnndc6f6ds4zcwdy82d6mv94xx2anch950",
    url: "https://ghost.coldstake.io/",
    is_active: false,
};
const DARKPAY: PrivatePool = PrivatePool {
    pubkey: "gcs1wmr49e2fd8p09dll8djdfsek6m5zuxhjdq3c4c",
    url: "https://ghost-pool.darkpay.market/",
    is_active: false,
};
const GHOST_STAKE: PrivatePool = PrivatePool {
    pubkey: "gcs1cxgy3uzz05djx5j0g40trxkweq7jwllkzzpkf5",
    url: "https://ghoststake.com/",
    is_active: false,
};
const NOVASTERA_EU: PrivatePool = PrivatePool {
    pubkey: "gcs1gujv3jjfec75pqkezjckskxw9cfvvdmjkw5g6x",
    url: "https://ghost.novastera.eu/",
    is_active: false,
};
const GHOST_CHEAP: PrivatePool = PrivatePool {
    pubkey: "gcs10hnqtq2uvm538n8xck5e7phyqgast5v2m5wv7f",
    url: "https://ghost.cheap/",
    is_active: false,
};
const GHOST_POOL_DOT_NET: PrivatePool = PrivatePool {
    pubkey: "gcs154p3rm80dg3arc5jak3293mkgk45uhfggej3da",
    url: "https://ghost-pool.net/",
    is_active: false,
};

pub const POOLS: [PrivatePool; 12] = [
    MYGHOST,
    MEGA,
    GHOSTCSP,
    SUPERGHOSTPOS,
    POOLGHOSTRUS,
    GHOSTAKE,
    COLDSTAKE_IO,
    DARKPAY,
    GHOST_STAKE,
    NOVASTERA_EU,
    GHOST_CHEAP,
    GHOST_POOL_DOT_NET,
];
