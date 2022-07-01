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

`near call <CONTRACT OWNER WALLET> set_user --args '{"wallet": "<USER WALLET>", "n": "<USER NAME>", "disc": "<USER DIRCORD>", "mail": "<USER EMAIL>", "interests": u8}' --accountId <CALLER WALLET>`


Change user discord:

`near call <CONTRACT OWNER WALLET> set_user_discord --args '{"discord": "<USER DIRCORD>"}' --accountId <CALLER WALLET>`


Change user email:

`near call <CONTRACT OWNER WALLET> set_user_email --args '{"email": "<USER EMAIL>"}' --accountId <CALLER WALLET>`


Change user interests:

`near call <CONTRACT OWNER WALLET> set_user_interests --args '{"interests": u8}' --accountId <CALLER WALLET>`


Change user name:

`near call <CONTRACT OWNER WALLET> set_user_name --args '{"name": "<USER NAME>"}' --accountId <CALLER WALLET>`


Add new experience (returns experience's code number):

`near call <CONTRACT OWNER WALLET> set_experience --args '{"experience_name": "<NAME>", "description": "<EXPERIENCE DESCRIPTION>", "url": "<VIDEO URL>", "reward": f64, "moment": "<COMMENT>", "time": u16, "expire_date": i64, "topic": u8}' --accountId <CALLER WALLET>`
optional: `--deposit <NEARS>`


Change moment comment of experience:

`near call <CONTRACT OWNER WALLET> set_moment_comment --args '{"video_n": u128, "comment": "<MOMENT COMMENT>"}' --accountId <CALLER WALLET>`


Change moment time of experience:

`near call <CONTRACT OWNER WALLET> set_moment_time --args '{"video_n": u128, "time": u16}' --accountId <CALLER WALLET>`


Change experience description:

`near call <CONTRACT OWNER WALLET> set_experience_description --args '{"video_n": u128, "description": "<EXPERIENCE DESCRIPTION>"}' --accountId <CALLER WALLET>`


Change experience expire date:

`near call <CONTRACT OWNER WALLET> set_experience_expire_date --args '{"video_n": u128, "date": i64}' --accountId <CALLER WALLET>`


Add PoV to experience:

`near call <CONTRACT OWNER WALLET> set_pov --args '{"video_n": u128, "wallet": "<WALLET OF POV GIVER>", "pov": "<COMMENT>"}' --accountId <CALLER WALLET>`

*************
** Getters **
*************

Get experience (returns struct Experience):

`near view <CONTRACT OWNER WALLET> get_experience --args '{"video_n": u128}'`


Get user (returns struct User):

`near view <CONTRACT OWNER WALLET> get_user --args '{"wallet": "<USER WALLET>"}'`

Get experience title:

`near view <CONTRACT OWNER WALLET> get_title --args '{"video_n": u128}'`


Get experience owner:

`near view <CONTRACT OWNER WALLET> get_exp_owner --args '{"video_n": u128}'`


Get experience description:

`near view <CONTRACT OWNER WALLET> get_exp_description --args '{"video_n": u128}'`


Get video url:

`near view <CONTRACT OWNER WALLET> get_url --args '{"video_n": u128}'`


Get experience topic:

`near view <CONTRACT OWNER WALLET> get_topic --args '{"video_n": u128}'`


Get experience reward:

`near view <CONTRACT OWNER WALLET> get_reward --args '{"video_n": <EXPERIENCE NUMBER>}'`


Get experience expiration date:

`near view <CONTRACT OWNER WALLET> get_expiration_date --args '{"video_n": u128}'`


Get moment coment:

`near view <CONTRACT OWNER WALLET> get_moment_coment --args '{"video_n": u128}'`


Get moment time:

`near view <CONTRACT OWNER WALLET> get_moment_time --args '{"video_n": u128}'`


Get points of view for a moment (returns vec<(wallet, pov)>):

`near view <CONTRACT OWNER WALLET> get_pov_of_vid --args '{"video_n": u128}'`


Get experience status:

`near view <CONTRACT OWNER WALLET> get_exp_status --args '{"video_n": u128}'`


Get experiences by topic :

`near view <CONTRACT OWNER WALLET> get_exp_by_topic --args '{"topic": u8}'`


Get user's name:

`near view <CONTRACT OWNER WALLET> get_user_name --args '{"wallet": "<USER WALLET>"}'`


Get user's discord:

`near view <CONTRACT OWNER WALLET> get_user_discord --args '{"wallet": "<USER WALLET>"}'`


Get user's email:

`near view <CONTRACT OWNER WALLET> get_user_email --args '{"wallet": "<USER WALLET>"}'`


Get user's interests:

`near view <CONTRACT OWNER WALLET> get_user_interests --args '{"wallet": "<USER WALLET>"}'`


Get user's experiences:

`near view <CONTRACT OWNER WALLET> get_user_exp --args '{"wallet": "<USER WALLET>"}'`


Get experiences the user left a point of view:

`near view <CONTRACT OWNER WALLET> get_user_exp_pov --args '{"wallet": "<USER WALLET>"}'`


Get user's date of last comment:

`near view <CONTRACT OWNER WALLET> get_user_date --args '{"wallet": "<USER WALLET>"}'`


Get total of experiences in the contract:

`near view <CONTRACT OWNER WALLET> get_number_of_experiences --accountId <CALLER WALLET>`


***
** Transfer Tokens **
***

Activate an experience:

`near call <CONTRACT> activate_experience --args '{"video_n": u128}' --accountId <CALLER WALLET> --deposit <NEAR>`


Pay reward to best PoV:

`near call <CONTRACT> pay_reward --args '{"experience_number": u128, "wallet": "<BEST POV WALLET>"}' --accountId <CALLER WALLET>`
