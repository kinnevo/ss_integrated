# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    redeploy.sh                                        :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: user <marvin@42.fr>                        +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2022/06/16 22:50:12 by user              #+#    #+#              #
#    Updated: 2022/06/16 23:16:18 by user             ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

rm ./neardev/dev-account.env;
cargo build --target wasm32-unknown-unknown --release;
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/SharingShard.wasm;
DEV_WALLET=`cat ./neardev/dev-account`;
near call $DEV_WALLET new --accountId $DEV_WALLET
