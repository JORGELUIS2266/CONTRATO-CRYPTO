#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Map, Vec, symbol_short, Address, vec};

#[contract]
pub struct Contract;

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Creador {
    pub username: String,
    pub email: String,
    pub redes: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Contenido {
    pub titulo: String,
    pub descripcion: String,
    pub url_archivo: String,
    pub autenticado: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct HistorialEliminacion {
    pub titulo: String,
    pub descripcion: String,
    pub timestamp: u64,
}

#[contractimpl]
impl Contract {
    pub fn registrar_creador(env: Env, nombre: String, correo: String, direccion_wallet: String) -> Result<(), u32> {
        let key = symbol_short!("CREADORES");
        let mut creadores: Map<String, Creador> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

        if creadores.contains_key(direccion_wallet.clone()) {
            return Err(1);
        }

        let nuevo = Creador { 
            username: nombre, 
            email: correo, 
            redes: None 
        };
        
        creadores.set(direccion_wallet.clone(), nuevo);
        env.storage().persistent().set(&key, &creadores);
        Ok(())
    }

    pub fn eliminar_creador(env: Env, direccion_wallet: String, confirmar: bool) -> Result<(), u32> {
        if !confirmar {
            return Err(2);
        }

        let key = symbol_short!("CREADORES");
        let mut creadores: Map<String, Creador> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));

        if !creadores.contains_key(direccion_wallet.clone()) {
            return Err(3);
        }

        creadores.remove(direccion_wallet);
        env.storage().persistent().set(&key, &creadores);
        Ok(())
    }

    pub fn alta_contenido(
        env: Env, 
        direccion_wallet: String, 
        titulo: String, 
        descripcion: String, 
        url_archivo: String
    ) -> Result<(), u32> {
        // Verificar si el creador existe
        let key_creadores = symbol_short!("CREADORES");
        let creadores: Map<String, Creador> = env.storage().persistent().get(&key_creadores).unwrap_or(Map::new(&env));
        
        if !creadores.contains_key(direccion_wallet.clone()) {
            return Err(5);
        }

        // Validar URL
        if url_archivo.len() <= 10 {
            return Err(6);
        }

        let key_contenido = symbol_short!("CONTENIDO");
        let mut contenidos: Map<String, Vec<Contenido>> = env.storage().persistent().get(&key_contenido).unwrap_or(Map::new(&env));

        let mut lista = contenidos.get(direccion_wallet.clone()).unwrap_or(Vec::new(&env));
        lista.push_back(Contenido {
            titulo,
            descripcion,
            url_archivo,
            autenticado: true,
        });

        contenidos.set(direccion_wallet, lista);
        env.storage().persistent().set(&key_contenido, &contenidos);
        Ok(())
    }

    pub fn consultar_contenido(env: Env, criterio: String) -> Vec<Contenido> {
        let key = symbol_short!("CONTENIDO");
        let contenidos: Map<String, Vec<Contenido>> = env.storage().persistent().get(&key).unwrap_or(Map::new(&env));
        let mut resultados = Vec::new(&env);
        let criterio_str = criterio.to_string();

        for (_, lista) in contenidos.iter() {
            for contenido in lista.iter() {
                if contenido.titulo.to_string().contains(&criterio_str) || 
                   contenido.descripcion.to_string().contains(&criterio_str) {
                    resultados.push_back(contenido.clone());
                }
            }
        }

        resultados
    }

    pub fn revisar_contenido(_env: Env, contenido: Contenido) -> bool {
        let palabras_prohibidas = vec![
            &_env,
            "pornografía",
            "violencia", 
            "odio"
        ];
        
        let descripcion = contenido.descripcion.to_string();
        !palabras_prohibidas.iter().any(|p| descripcion.contains(p))
    }
}