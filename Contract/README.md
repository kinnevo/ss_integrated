# SharingShard

***
** Deploying contract **
***
Build contract:

`cargo build --target wasm32-unknown-unknown --release`

Use near-cli to deploy the smart contract to NEAR test network:

`near deploy --wasmFile target/wasm32-unknown-unknown/release/SharingShard.wasm --accountId <YOUR_ACCOUNT_HERE>`


***
** Initializing **
***

Initializing contract:

`near call <YOUR_ACCOUNT_HERE> new --accountId <YOUR_ACCOUNT_HERE>`

***
** Setters **
***

Add new user:

`near call <CONTRACT OWNER WALLET> setUser --args '{"wallet": "<USER WALLET>", "n": "<USER NAME>", "disc": "<USER DIRCORD>", "mail": "<USER EMAIL>", "interst": <CODE NUMBER FOR USER ITERESTS>}' --accountId <CALLER WALLET>`


Add new experience (returns experience's code number):

`near call <CONTRACT OWNER WALLET> setExperience --args '{"wallet": "<USER WALLET>", "experience_name": "<NAME>", "description": "<EXPERIENCE DESCRIPTION>", "url": "<VIDEO URL>", "reward": f64, "expire_date": <EXPIRATION DATE>, "topic": u8}' --accountId <CALLER WALLET>`


Add moment to experience:

`near call <CONTRACT OWNER WALLET> setMoment --args '{"wallet": "<USER WALLET>", "experience_number": u128, "time": u16, "comment": "<MOMENT COMMENT>"}' --accountId <CALLER WALLET>`


Add PoV to experience:

`near call <CONTRACT OWNER WALLET> setPov --args '{"video_n": u128, "wallet": "<WALLET OF POV GIVER>", "pov": "<COMMENT>"}' --accountId <CALLER WALLET>`

*************
** Getters **
*************

Get experience (returns struct Experience):

`near view <CONTRACT OWNER WALLET> getExperience --args '{"video_n": u128}'`


Get user (returns struct User):

`near view <CONTRACT OWNER WALLET> getUser --args '{"wallet": "<USER WALLET>"}'`

Get experience title:

`near view <CONTRACT OWNER WALLET> getTitle --args '{"video_n": u128}'`


Get experience owner:

`near view <CONTRACT OWNER WALLET> getExp_owner --args '{"video_n": u128}'`


Get experience description:

`near view <CONTRACT OWNER WALLET> getExp_description --args '{"video_n": u128}'`


Get video url:

`near view <CONTRACT OWNER WALLET> getUrl --args '{"video_n": u128}'`


Get experience topic:

`near view <CONTRACT OWNER WALLET> getTopic --args '{"video_n": u128}'`


Get experience reward:

`near view <CONTRACT OWNER WALLET> getReward --args '{"video_n": <EXPERIENCE NUMBER>}'`


Get experience expiration date:

`near view <CONTRACT OWNER WALLET> getExpiration_date --args '{"video_n": u128}'`


Get moment coment:

`near view <CONTRACT OWNER WALLET> getMoment_coment --args '{"video_n": u128}'`


Get moment time:

`near view <CONTRACT OWNER WALLET> getMoment_time --args '{"video_n": u128}'`


Get points of view for a moment (returns vec<(wallet, pov)>):

`near view <CONTRACT OWNER WALLET> getPov_of_vid --args '{"video_n": u128}'`


Get experience status:

`near view <CONTRACT OWNER WALLET> getExp_status --args '{"video_n": u128}'`


Get experiences by topic :

`near view <CONTRACT OWNER WALLET> getExp_by_topic --args '{"topic": u8}'`


Get user's name:

`near view <CONTRACT OWNER WALLET> getUser_name --args '{"wallet": "<USER WALLET>"}'`


Get user's discord:

`near view <CONTRACT OWNER WALLET> getUser_discord --args '{"wallet": "<USER WALLET>"}'`


Get user's email:

`near view <CONTRACT OWNER WALLET> getUser_email --args '{"wallet": "<USER WALLET>"}'`


Get user's interests:

`near view <CONTRACT OWNER WALLET> getUser_interests --args '{"wallet": "<USER WALLET>"}'`


Get user's experiences:

`near view <CONTRACT OWNER WALLET> getUser_exp --args '{"wallet": "<USER WALLET>"}'`


Get experiences the user left a point of view:

`near view <CONTRACT OWNER WALLET> getUser_exp_pov --args '{"wallet": "<USER WALLET>"}'`


Get user's date of last comment:

`near view <CONTRACT OWNER WALLET> getUser_date --args '{"wallet": "<USER WALLET>"}'`


Get total of experiences in the contract:

`near view <CONTRACT OWNER WALLET> getNumber_of_experiences --accountId <CALLER WALLET>`
