use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use serde::{Deserialize, Serialize};

const YOCTO_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
const SEND_FUNDS: Balance = 4_500_000_000_000_000_000;
//https://docs.near.org/docs/concepts/storage-staking
//const STORAGE_PER_BYTE: Balance = 10_000_000_000_000_000_000;
const FEE: f64 = 1.1;

/*
** Structures
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Experience_json{
    title: String,
    owner: AccountId,
    description: String,
    url: String,
    topic: u8,
    reward: f64,
    exp_date: i64,
    moment: String,
    time: u16,
    pov: Vec<(AccountId, String)>,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User_json{
    name: String,
    discord: String,
    email: String,
    interests: u8,
    my_exp: Vec<u128>,
    pov_exp: Vec<u128>,
    date: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Experience{
    title: String,
    owner: AccountId,
    description: String,
    url: String,
    topic: u8,
    reward: f64,
    exp_date: i64,
    moment: String,
    time: u16,
    pov: UnorderedMap<AccountId, String>,
    status: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct User{
    name: String,
    discord: String,
    email: String,
    interests: u8,
    my_exp: Vector<u128>,
    pov_exp: Vector<u128>,
    date: i64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract{
    users: LookupMap<AccountId, User>,
    experience: LookupMap<u128, Experience>,
    exp_by_topic: LookupMap< u8, Vec<u128> >,
    n_exp: u128,
    ss_wallet: AccountId,
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
            ss_wallet: "jciglesias.testnet".parse().unwrap(), //to change wallet
        }
    }

    pub fn pay_reward(&mut self, experience_number: u128, wallet: AccountId){
        let caller = env::signer_account_id();
        self.verify_exp_owner(experience_number.clone(), caller.clone());
        assert_eq!(self.getExp_status(experience_number.clone()),
        "Active".to_string(), "Experience not active");
        assert_ne!(self.experience.get(
            &experience_number.clone()).unwrap().pov.get(&wallet.clone()),
            None,
            "{} did not give a PoV for this experience", wallet.clone());
        Promise::new(wallet).transfer(
            (self.getReward(experience_number.clone()) as Balance)
            * YOCTO_NEAR);
        let mut exp = self.experience.get(&experience_number.clone()).unwrap();
        exp.status = "Closed".to_string();
        self.experience.insert(&experience_number.clone() , &exp);
    }

    fn send_fee(&self, deposit: Balance, reward: f64, wallet: AccountId){
        let fee = ((reward * FEE) - reward) as u128 * YOCTO_NEAR;
        Promise::new(self.ss_wallet.clone()).transfer(fee);
        let diff = deposit - ((reward as u128 * YOCTO_NEAR) + fee);
        if diff > SEND_FUNDS{
            Promise::new(wallet).transfer(diff);
        }
    }
/*
** Setters
*/
    //#[derive(BorshStorageKey)]
    pub fn setUser(&mut self,
        wallet: AccountId,
        n: String,
        disc: String,
        mail: String,
        interests: u8){
        assert!(!self.users.contains_key(&wallet.clone()), "User already exists");
        self.users.insert(&wallet.clone(), &User{name: n,
            discord: disc,
            email: mail,
            interests: interests,
            my_exp: Vector::new(b"m"),
            pov_exp: Vector::new(b"m"),
            date: 0});
    }

    pub fn setUser_discord(&mut self, discord: String){
        let wallet = env::signer_account_id();
        self.verify_user(wallet.clone());
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.discord = discord;
        self.users.insert(&wallet, &user);
    }

    pub fn setUser_email(&mut self, email: String){
        let wallet = env::signer_account_id();
        self.verify_user(wallet.clone());
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.email = email;
        self.users.insert(&wallet, &user);
    }
    
    pub fn setUser_interests(&mut self, interests: u8){
        let wallet = env::signer_account_id();
        self.verify_user(wallet.clone());
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.interests = interests;
        self.users.insert(&wallet, &user);
    }

    pub fn setUser_name(&mut self, name: String){
        let wallet = env::signer_account_id();
        self.verify_user(wallet.clone());
        let mut user = self.users.get(&wallet.clone()).unwrap();
        user.name = name;
        self.users.insert(&wallet, &user);
    }

    #[payable]
    pub fn setExperience(&mut self,
        experience_name: String,
        description: String,
        url: String,
        reward: f64,
        moment: String,
        time: u16,
        expire_date: i64,
        topic: u8) ->u128{
        self.verify_user(env::signer_account_id());
        let mut stat = "In process".to_string();
        if env::attached_deposit() > 0 {
            assert!(env::attached_deposit() >= ((reward * FEE) as u128 * YOCTO_NEAR),
            "Wrong amount of NEARs");
            self.send_fee(env::attached_deposit(), reward.clone(),
            env::signer_account_id());
            stat = "Active".to_string();
        }
        self.n_exp += 1;
        self.experience.insert(&self.n_exp.clone(),
        &Experience{title: experience_name.clone(),
            owner: env::signer_account_id(),
            description: description,
            url: url,
            reward: reward,
            moment: moment,
            time : time,
            pov: UnorderedMap::new(b"m"),
            topic: topic.clone(),
            exp_date: expire_date,
            status: stat});
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
        let mut usr = self.users.get(&env::signer_account_id()).unwrap();
        usr.my_exp.push(&self.n_exp.clone());
        self.users.insert(&env::signer_account_id(), &usr);
        self.n_exp
    }

    pub fn setMoment_comment(&mut self, video_n: u128, comment: String){
        self.verify_exp(video_n.clone());
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.moment = comment;
        self.experience.insert(&video_n.clone(), &exp);
    }

    pub fn setMomment_time(&mut self, video_n: u128, time: u16){
        self.verify_exp(video_n.clone());
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.time = time;
        self.experience.insert(&video_n.clone(), &exp);
    }

    pub fn setExperience_description(&mut self, video_n: u128, description: String){
        self.verify_exp(video_n.clone());
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.description = description;
        self.experience.insert(&video_n.clone(), &exp);
    }

    pub fn setExperience_expire_date(&mut self, video_n: u128, date: i64){
        self.verify_exp(video_n.clone());
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        assert_eq!(self.experience.get(&video_n.clone()).unwrap().status.clone(),
        "In process".to_string(), "Experience not in process");
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.exp_date = date;
        self.experience.insert(&video_n.clone(), &exp);
    }

    #[payable]
    pub fn activateExperience(&mut self, video_n: u128){
        self.verify_user(env::signer_account_id());
        self.verify_exp(video_n.clone());
        assert_eq!(self.experience.get(&video_n.clone()).unwrap().status.clone(),
        "In process".to_string(), "Experience already activated");
        self.verify_exp_owner(video_n.clone(), env::signer_account_id());
        let reward = self.experience.get(&video_n.clone()).unwrap().reward.clone();
        assert!(env::attached_deposit() >= ((reward * FEE) as u128 * YOCTO_NEAR),
        "Not enough tokens");
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        exp.status = "Active".to_string();
        self.experience.insert(&video_n.clone(), &exp);
        self.send_fee(env::attached_deposit(), reward.clone(),
        env::signer_account_id());
    }

    pub fn setPov(&mut self, video_n: u128, wallet: AccountId, pov: String){
        self.verify_exp(video_n.clone());
        self.verify_user(wallet.clone());
        let mut exp = self.experience.get(&video_n.clone()).unwrap();
        assert_eq!(exp.pov.get(&wallet.clone()), None,
        "User has already given a pov for this experience");
        exp.pov.insert(&wallet.clone(), &pov);
        self.experience.insert(&video_n.clone(), &exp);
        let mut usr = self.users.get(&wallet.clone()).unwrap();
        usr.pov_exp.push(&video_n.clone());
        self.users.insert(&wallet.clone(), &usr);
    }
/*
** Getters
*/
    pub fn getExperience(&self, video_n: u128) ->Experience_json{
        self.verify_exp(video_n.clone());
        let exp = self.experience.get(&video_n.clone()).unwrap();
        Experience_json{title: exp.title,
            owner: exp.owner,
            description: exp.description,
            url: exp.url,
            reward: exp.reward,
            moment: exp.moment,
            time : exp.time,
            pov: exp.pov.to_vec(),
            topic: exp.topic,
            exp_date: exp.exp_date,
            status: exp.status
        }
    }

    pub fn getUser(&self, wallet: AccountId) ->User_json{
        self.verify_user(wallet.clone());
        let user = self.users.get(&wallet).unwrap();
        User_json{
            name: user.name,
            discord: user.discord,
            email: user.email,
            interests: user.interests,
            my_exp: user.my_exp.to_vec(),
            pov_exp: user.pov_exp.to_vec(),
            date: user.date
        }
    }

    pub fn getTitle(&self, video_n: u128) ->String{
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap().title
    }
    
    pub fn getExp_owner(&self, video_n: u128) ->AccountId{
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap().owner
    }

    pub fn getExp_description(&self, video_n: u128) -> String{
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap().description
    }

    pub fn getUrl(&self, video_n: u128) -> String{
        self.verify_exp(video_n.clone());
        let exp = self.experience.get(&video_n.clone()).unwrap();
        exp.url
    }

    pub fn getTopic(&self, video_n: u128) -> u8 {
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n.clone()).unwrap().topic
    }

    pub fn getReward(&self, video_n: u128) -> f64{
        self.verify_exp(video_n.clone());
        (self.experience.get(&video_n.clone())).unwrap().reward
    }

    pub fn getExpiration_date(&self, video_n: u128) ->i64{
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().exp_date
    }

    pub fn getMoment_coment(&self, video_n: u128) ->String{
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().moment
    }

    pub fn getMoment_time(&self, video_n: u128) ->u16{
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().time
    }

    pub fn getPov_of_vid(&self, video_n: u128) ->Vec<(AccountId,String)>{
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().pov.to_vec()
    }

    pub fn getExp_status(&self, video_n: u128) ->String{
        self.verify_exp(video_n.clone());
        self.experience.get(&video_n).unwrap().status
    }

    pub fn getExp_by_topic(&self, topic: u8) ->Vec<u128>{
        self.exp_by_topic.get(&topic).unwrap()
    }

    pub fn getUser_name(&self, wallet: AccountId) ->String{
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().name
    }

    pub fn getUser_discord(&self, wallet: AccountId) ->String{
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().discord
    }

    pub fn getUser_email(&self, wallet: AccountId) ->String{
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().email
    }

    pub fn getUser_interests(&self, wallet: AccountId) ->u8{
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().interests
    }

    pub fn getUser_exp(&self, wallet: AccountId) ->Vec<u128>{
        self.verify_user(wallet.clone());
        let usr = self.users.get(&wallet.clone()).unwrap();
        usr.my_exp.to_vec()
    }

    pub fn getUser_exp_pov(&self, wallet: AccountId) ->Vec<u128>{
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().pov_exp.to_vec()
    }

    pub fn getUser_date(&self, wallet: AccountId) ->i64{
        self.verify_user(wallet.clone());
        self.users.get(&wallet).unwrap().date
    }

    pub fn getNumber_of_experiences(&self) ->u128{
        self.n_exp
    }
/*
** Verifiers
*/
    fn verify_exp(&self, video_n: u128){
        assert!(self.experience.contains_key(&video_n.clone()),
        "Experience number {} does not exist", video_n);
    }

    fn verify_exp_owner(&self, video_n: u128, wallet: AccountId){
        assert_eq!(self.experience.get(&video_n.clone()).unwrap().owner.clone(),
        wallet.clone(), "{} is not the owner of the experience", wallet.clone());
    }

    fn verify_user(&self, wallet: AccountId){
        assert!(self.users.contains_key(&wallet.clone()),"No user for this wallet");
    }
}

fn main() {
    let mut contract = Contract::new();
    let id: AccountId = "pepe.near".parse().unwrap();
    let id2: AccountId = "bob.near".parse().unwrap();
    contract.setUser(id.clone(),
    "pepe".to_string(),
    "pepediscord".to_string(),
    "pepemail".to_string(), 8);
    for n in 1..20{
        contract.setExperience(
        "experience 1".to_string(),
        "descripcion video pepe".to_string(),
        "https://video.de/pepe".to_string(),
        12.0,
        "pepe moment".to_string(),
        100,
        1200,
        2);
    }
    contract.setUser(id2.clone(),
    "bob".to_string(),
    "bobdiscord".to_string(),
    "bobmail".to_string(), 7);
    let exp = contract.setExperience(
    "experience 2".to_string(),
    "descripcion video bob".to_string(),
    "https://video.de/bob".to_string(),
    20.0,
    "bob moment".to_string(),
    50,
    100,
    2);
    //contract.setMoment(id2.clone(), exp.clone(), 120, "bob moment".to_string());
    let rew = contract.getReward(1);
    contract.setPov(1, id2.clone(), "first pov".to_string());
    contract.setPov(1, id.clone(), "second pov".to_string());
    let exp_tmp = contract.getExperience(1);
    let usr_tmp = contract.getUser(id.clone());
    println!("{:?}", usr_tmp);
    println!("{:?}", exp_tmp);
    //let exp_encoded = exp.try_to_vec().unwrap();
    //println!("experience encoded = {:?}", exp_encoded.clone());
    //let exp_decoded = Experience::try_from_slice(&exp_encoded).unwrap();
    //println!("experience decoded = {:?}", exp_decoded);
    println!("reward for experience 1 = {:?}", rew);
    println!("url = {}", contract.getUrl(1));
    println!("{} experience title = {:?}", exp, contract.getTitle(exp));
    println!("{} experience description = {:?}", exp, contract.getExp_description(exp));
    println!("{} experience video url = {:?}", exp, contract.getUrl(exp));
    println!("{} experience topic = {:?}", exp, contract.getTopic(exp));
    println!("{} experience reward = {:?}", exp, contract.getReward(exp));
    println!("{} experience expiration date = {:?}",
    exp, contract.getExpiration_date(exp));
    println!("{} experience moment comment = {:?}",
    exp, contract.getMoment_coment(exp));
    println!("{} experience moment time = {:?}", exp, contract.getMoment_time(exp));
    println!("{} experience points of view = {:?}",
    exp, contract.getPov_of_vid(exp));
    println!("pepe's experiences = {:?}", contract.getUser_exp(id.clone()));
    println!("experiences on area 2 = {:?}", contract.getExp_by_topic(2));
    println!("{} user name = {:?}", id, contract.getUser_name(id.clone()));
    println!("{} user discord = {:?}", id, contract.getUser_discord(id.clone()));
    println!("{} user email = {:?}", id, contract.getUser_email(id.clone()));
    println!("{} user interests = {:?}",
    id, contract.getUser_interests(id.clone()));
    println!("experiences {} has left a pov = {:?}",
    id.clone(), contract.getUser_exp_pov(id.clone()));
    println!("last date {} commented = {:?}",
    id.clone(), contract.getUser_date(id.clone()));
    println!("total of experiences = {}", contract.getNumber_of_experiences());
}