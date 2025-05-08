#![cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String, Address, Vec, testutils::Address as _};

    #[test]
    fn test_registro_creador() {
        let env = Env::default();
        let contract = Contract;
        let wallet = String::from_slice(&env, "wallet1");

        // Test registro exitoso
        assert_eq!(
            contract.registrar_creador(
                env.clone(),
                String::from_slice(&env, "Alice"),
                String::from_slice(&env, "alice@example.com"),
                wallet.clone()
            ),
            Ok(())
        );

        // Test duplicado debe fallar
        assert_eq!(
            contract.registrar_creador(
                env.clone(),
                String::from_slice(&env, "Alice2"),
                String::from_slice(&env, "alice2@example.com"),
                wallet
            ),
            Err(1)
        );
    }

    #[test]
    fn test_alta_contenido() {
        let env = Env::default();
        let contract = Contract;
        let wallet = String::from_slice(&env, "wallet2");

        // Primero registrar el creador
        contract.registrar_creador(
            env.clone(),
            String::from_slice(&env, "Bob"),
            String::from_slice(&env, "bob@example.com"),
            wallet.clone()
        ).unwrap();

        // Test alta de contenido exitosa
        assert_eq!(
            contract.alta_contenido(
                env.clone(),
                wallet.clone(),
                String::from_slice(&env, "Título"),
                String::from_slice(&env, "Descripción"),
                String::from_slice(&env, "https://example.com/1234567890")
            ),
            Ok(())
        );

        // Test URL muy corta debe fallar
        assert_eq!(
            contract.alta_contenido(
                env.clone(),
                wallet,
                String::from_slice(&env, "Título"),
                String::from_slice(&env, "Descripción"),
                String::from_slice(&env, "short")
            ),
            Err(6)
        );
    }

    #[test]
    fn test_revision_contenido() {
        let env = Env::default();
        let contract = Contract;

        let contenido_bueno = Contenido {
            titulo: String::from_slice(&env, "Bueno"),
            descripcion: String::from_slice(&env, "Contenido apropiado"),
            url_archivo: String::from_slice(&env, "http://example.com"),
            autenticado: true,
        };

        let contenido_malo = Contenido {
            titulo: String::from_slice(&env, "Malo"),
            descripcion: String::from_slice(&env, "Contiene violencia"),
            url_archivo: String::from_slice(&env, "http://example.com"),
            autenticado: true,
        };

        assert!(contract.revisar_contenido(env.clone(), contenido_bueno));
        assert!(!contract.revisar_contenido(env.clone(), contenido_malo));
    }
}