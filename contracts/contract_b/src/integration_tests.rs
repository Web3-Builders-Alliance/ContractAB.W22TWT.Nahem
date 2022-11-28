#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, Addr, Empty};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, GetAdminResponse, InstantiateMsg, QueryMsg};

    const USER: &str = "user";
    const ADMIN: &str = "admin";

    // Contract usually wraps a message but this time we'll use an Empty message
    fn contract_b() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn proper_instantiation() {
        let mut app = App::default();

        // simulates uploading the contract to the blockchain to get the code_id (to instantiate the contract later)
        let code_id = app.store_code(contract_b());

        // instantiates contract, unwrap() guarantees success
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(ADMIN),
                &InstantiateMsg {},
                &[],
                "Contract B",
                None,
            )
            .unwrap();

        // query contract admin
        let resp: GetAdminResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::GetAdmin {})
            .unwrap();

        // assert that the queried contract admin is actually "admin"
        assert_eq!(resp.admin, Addr::unchecked(ADMIN));
    }

    #[test]
    fn withdraw_funds() {
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(ADMIN), coins(1_000_000, "uatom"))
                .unwrap()
        });

        // simulates uploading the contract to the blockchain to get the code_id (to instantiate the contract later)
        let code_id = app.store_code(contract_b());

        // instantiates contract, unwrap() guarantees successful instantiation
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(ADMIN),
                &InstantiateMsg {},
                &coins(1_000_000, "uatom"),
                "Contract B",
                None,
            )
            .unwrap();

        // withdraw funds to user address, unwrap() guarantees successful execution
        app.execute_contract(
            Addr::unchecked(ADMIN),
            contract_addr.clone(),
            &ExecuteMsg::Withdraw { to_address: None },
            &[],
        )
        .unwrap();

        let contract_balance = app
            .wrap()
            .query_all_balances(Addr::unchecked(contract_addr))
            .unwrap();

        let admin_balance = app
            .wrap()
            .query_all_balances(Addr::unchecked(ADMIN))
            .unwrap();

        assert_eq!(contract_balance, []);
        assert_eq!(admin_balance, coins(1_000_000, "uatom"));
    }

    #[test]
    fn withdraw_funds_to_user_address() {
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(ADMIN), coins(1_000_000, "uatom"))
                .unwrap()
        });

        // simulates uploading the contract to the blockchain to get the code_id (to instantiate the contract later)
        let code_id = app.store_code(contract_b());

        // instantiates contract, unwrap() guarantees successful instantiation
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked(ADMIN),
                &InstantiateMsg {},
                &coins(1_000_000, "uatom"),
                "Contract B",
                None,
            )
            .unwrap();

        // withdraw funds to user address, unwrap() guarantees successful execution
        app.execute_contract(
            Addr::unchecked(ADMIN),
            contract_addr.clone(),
            &ExecuteMsg::Withdraw {
                to_address: Some(USER.to_string()),
            },
            &[],
        )
        .unwrap();

        let contract_balance = app
            .wrap()
            .query_all_balances(Addr::unchecked(contract_addr))
            .unwrap();

        let admin_balance = app
            .wrap()
            .query_all_balances(Addr::unchecked(ADMIN))
            .unwrap();

        let user_balance = app
            .wrap()
            .query_all_balances(Addr::unchecked(USER))
            .unwrap();

        assert_eq!(contract_balance, []);
        assert_eq!(admin_balance, []);
        assert_eq!(user_balance, coins(1_000_000, "uatom"));
    }
}
