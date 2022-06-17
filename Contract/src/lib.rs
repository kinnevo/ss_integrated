use near_sdk::{env, near_bindgen, AccountId, Balance, Gas, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::collections::{LookupMap, UnorderedMap};

/*
** Structures
*/
#[derive(Debug)]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Experience{
    title: String,
    owner: AccountId,
    description: String,
    url: String,
    topic: u8,
    reward: u128,
    exp_date: i64,
    moment: String,
    time: u16,
    pov: UnorderedMap<AccountId, String>,
}
#[derive(Debug)]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct User{
    name: String,
    discord: String,
    email: String,
    interests: u8,
    my_exp: Vec<u128>,
    pov_exp: Vec<u128>,
    date: i64,
}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract{
    users: LookupMap<AccountId, User>,
    experience: LookupMap<u128, Experience>,
    exp_by_topic: LookupMap< u8, Vec<u128> >,
    n_exp: u128,
    fee: u128,
}

/*
** Functions
*/

/*
** Initialization
*/
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self{
            users: LookupMap::new(b"m"),
            experience: LookupMap::new(b"m"),
            exp_by_topic: LookupMap::new(b"m"),
            n_exp: 0,
            fee: 10,
        }
    }
/*
** Setters
*/
    pub fn new_user(&mut self,
        wallet: AccountId,
        n: String,
        disc: String,
        mail: String,
        interst: u8){
        assert!(!self.users.contains_key(&wallet.clone()), "User already exists");
        self.users.insert(&wallet.clone(), &User{name: n,
            discord: disc,
            email: mail,
            interests: interst,
            my_exp: Vec::new(),
            pov_exp: Vec::new(),
            date: 0});
    }
    
    pub fn add_experience(&mut self,
        wallet: AccountId,
        experience_name: String,
        description: String,
        url: String,
        reward: u128,
        expire_date: i64,
        topic: u8) ->u128{
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
        self.n_exp += 1;
        self.experience.insert(&self.n_exp.clone(),
        &Experience{title: experience_name.clone(),
            owner: wallet.clone(),
            description: description,
            url: url,
            reward: reward,
            moment: "".to_string(),
            time : 0,
            pov: UnorderedMap::new(b"m"),
            topic: topic.clone(),
            exp_date: expire_date
        });
        if self.exp_by_topic.contains_key(&topic.clone()){
            let mut vec = self.exp_by_topic.get(&topic.clone()).unwrap();
            vec.push(self.n_exp.clone());
            self.exp_by_topic.insert(&topic.clone(), &vec);
        }
        else{
            self.exp_by_topic.insert(&topic.clone(), &Vec::new());
            let mut vec = self.exp_by_topic.get(&topic.clone()).unwrap();
            vec.push(self.n_exp.clone());
            self.exp_by_topic.insert(&topic.clone(), &vec);
        }
        let mut usr = self.users.get(&wallet.clone()).unwrap();
        usr.my_exp.push(self.n_exp.clone());
        self.users.insert(&wallet.clone(), &usr);
        self.n_exp
    }

    pub fn add_moment(&mut self,
        wallet: AccountId,
        experience_number: u128,
        time: u16,
        comment: String){
        assert!(self.experience.contains_key(&experience_number.clone()),
        "Experience number {} does not exist", experience_number);
        let mut exp = self.experience.get(&experience_number.clone()).unwrap();
        if exp.owner == wallet{
            exp.moment = comment;
            exp.time = time;
            self.experience.insert(&experience_number, &exp);
        }
    }

    pub fn add_pov(&mut self, video_n: u128, wallet: AccountId, pov: String){
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not esxist", video_n);
        assert!(self.users.contains_key(&wallet.clone()), "User does not exist");
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        assert_eq!(exp.pov.get(&wallet.clone()), None,
        "User has already given a pov for this experience");
        exp.pov.insert(&wallet.clone(), &pov);
        self.experience.insert(&video_n.clone(), &exp);
        let mut usr = self.users.get(&wallet.clone()).unwrap();
        usr.pov_exp.push(video_n.clone());
        self.users.insert(&wallet.clone(), &usr);
    }
/*
** Getters
*/
    #[result_serializer(borsh)]
    pub fn get_experience(&self, video_n: u128) ->Experience{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        self.experience.get(&video_n.clone()).unwrap()
    }

    #[result_serializer(borsh)]
    pub fn get_user(&self, wallet: AccountId) ->User{
        assert!(self.users.contains_key(&wallet.clone()), "No such user");
        self.users.get(&wallet).unwrap()
    }

    pub fn get_title(&self, video_n: u128) ->String{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        self.experience.get(&video_n.clone()).unwrap().title
    }
    
    pub fn get_description(&self, video_n: u128) -> String{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        self.experience.get(&video_n.clone()).unwrap().description
    }

    pub fn get_url(&self, video_n: u128) -> String{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        let exp = self.experience.get(&video_n.clone()).unwrap();
        exp.url
    }

    pub fn get_topic(&self, video_n: u128) -> u8 {
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        self.experience.get(&video_n.clone()).unwrap().topic
    }

    pub fn get_reward(&self, video_n: u128) -> u128{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        (self.experience.get(&video_n.clone())).unwrap().reward
    }

    pub fn get_expiration_date(&self, video_n: u128) ->i64{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        self.experience.get(&video_n).unwrap().exp_date
    }

    pub fn get_moment_coment(&self, video_n: u128) ->String{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        self.experience.get(&video_n).unwrap().moment
    }

    pub fn get_moment_time(&self, video_n: u128) ->u16{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        self.experience.get(&video_n).unwrap().time
    }

    #[result_serializer(borsh)]
    pub fn get_pov_of_vid(&self, video_n: u128) ->UnorderedMap<AccountId,String>{
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
        self.experience.get(&video_n).unwrap().pov
    }

    pub fn get_exp_by_topic(&self, topic: u8) ->Vec<u128>{
        self.exp_by_topic.get(&topic).unwrap()
    }

    pub fn get_user_name(&self, wallet: AccountId) ->String{
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
        self.users.get(&wallet).unwrap().name
    }

    pub fn get_user_discord(&self, wallet: AccountId) ->String{
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
        self.users.get(&wallet).unwrap().discord
    }

    pub fn get_user_email(&self, wallet: AccountId) ->String{
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
        self.users.get(&wallet).unwrap().email
    }

    pub fn get_user_interests(&self, wallet: AccountId) ->u8{
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
        self.users.get(&wallet).unwrap().interests
    }

    pub fn get_user_exp(&self, wallet: AccountId) ->Vec<u128>{
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
        let usr = self.users.get(&wallet.clone()).unwrap();
        usr.my_exp
    }

    pub fn get_user_exp_pov(&self, wallet: AccountId) ->Vec<u128>{
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
        self.users.get(&wallet).unwrap().pov_exp
    }

    pub fn get_user_date(&self, wallet: AccountId) ->i64{
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
        self.users.get(&wallet).unwrap().date
    }

    pub fn get_number_of_experiences(&self) ->u128{
        self.n_exp
    }
}

fn main() {
    let mut contract = Contract::new();
    let id: AccountId = "pepe.near".parse().unwrap();
    let id2: AccountId = "bob.near".parse().unwrap();
    contract.new_user(id.clone(),
    "pepe".to_string(),
    "pepediscord".to_string(),
    "pepemail".to_string(), 8);
    for n in 1..20{
        contract.add_experience(id.clone(),
        "experience 1".to_string(),
        "descripcion video pepe".to_string(),
        "https://video.de/pepe".to_string(), 12, 1200, 2);
    }
    contract.new_user(id2.clone(),
    "bob".to_string(),
    "bobdiscord".to_string(),
    "bobmail".to_string(), 7);
    let exp = contract.add_experience(id2.clone(),
    "experience 2".to_string(),
    "descripcion video bob".to_string(),
    "https://video.de/bob".to_string(), 20, 100, 2);
    contract.add_moment(id2.clone(), exp.clone(), 120, "bob moment".to_string());
    let rew = contract.get_reward(1);
    contract.add_pov(1, id2.clone(), "first pov".to_string());
    contract.add_pov(1, id.clone(), "second pov".to_string());
    let exp_tmp = contract.get_experience(1);
    let usr_tmp = contract.get_user(id.clone());
    println!("{:?}", usr_tmp);
    println!("{:?}", exp_tmp);
    println!("reward for experience 1 = {:?}", rew);
    println!("url = {}", contract.get_url(1));
    println!("{} experience title = {:?}", exp, contract.get_title(exp));
    println!("{} experience description = {:?}", exp, contract.get_description(exp));
    println!("{} experience video url = {:?}", exp, contract.get_url(exp));
    println!("{} experience topic = {:?}", exp, contract.get_topic(exp));
    println!("{} experience reward = {:?}", exp, contract.get_reward(exp));
    println!("{} experience expiration date = {:?}",
    exp, contract.get_expiration_date(exp));
    println!("{} experience moment comment = {:?}",
    exp, contract.get_moment_coment(exp));
    println!("{} experience moment time = {:?}", exp, contract.get_moment_time(exp));
    println!("{} experience points of view = {:?}",
    exp, contract.get_pov_of_vid(exp));
    println!("pepe's experiences = {:?}", contract.get_user_exp(id.clone()));
    println!("experiences on area 2 = {:?}", contract.get_exp_by_topic(2));
    println!("{} user name = {:?}", id, contract.get_user_name(id.clone()));
    println!("{} user discord = {:?}", id, contract.get_user_discord(id.clone()));
    println!("{} user email = {:?}", id, contract.get_user_email(id.clone()));
    println!("{} user interests = {:?}",
    id, contract.get_user_interests(id.clone()));
    println!("experiences {} has left a pov = {:?}",
    id.clone(), contract.get_user_exp_pov(id.clone()));
    println!("last date {} commented = {:?}",
    id.clone(), contract.get_user_date(id.clone()));
    println!("total of experiences = {}", contract.get_number_of_experiences());
}