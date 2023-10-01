# mpago
![mpago-logo](https://github.com/vipago/mpago/assets/92828847/d8096f16-6b2a-4f55-a7fb-684d50de3dd1)

Uma SDK simples em rust para o mercado pago
# Como utilizar
Para começar a utilizar o `mpago`, crie um `mpago::client::MercadoPagoClient` com o access token:
```rs
use mpago::client::MercadoPagoClientBuilder;
let access_token = std::env::var("MERCADOPAGO_ACCESS_TOKEN").expect("mercado pago access token to be set");
let mp_client = MercadoPagoClientBuilder::builder(&access_token).build();
```

A API desta biblioteca utiliza muito o design pattern de **builder** para criar o corpo das requesições.
Aqui está um exemplo de como criar um pagamento usando o `mpago`:
```rs
mpago::payments::PaymentCreateBuilder(PaymentCreateOptions {
    transaction_amount: amount_in_brl,
    date_of_expiration: Some(date_of_expiration),
    ..Default::default()
})
.send(&mp_client)
.await?;
```

Para mais detalhes sobre a API do `mpago`, clone a biblioteca e rode `cargo doc --open` para abrir a documentação completa.
