# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    redeploy.sh                                        :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: user <marvin@42.fr>                        +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2022/06/16 22:50:12 by user              #+#    #+#              #
#    Updated: 2022/06/17 19:41:04 by user             ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

cargo build --target wasm32-unknown-unknown --release;
rm ./neardev/dev-account.env;
near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/SharingShard.wasm;
DEV_WALLET=`cat ./neardev/dev-account`;
near call $DEV_WALLET new --accountId $DEV_WALLET
