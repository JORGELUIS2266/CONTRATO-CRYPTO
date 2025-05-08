#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Map, Vec, symbol_short, Option};

#[contract]
pub struct Contract;

#[derive(Clone)]
#[contracttype]
pub struct Creador {
    pub username: String,
    pub email: String,
    pub redes: Option<String>,
}

#[derive(Clone)]
#[contracttype]
pub struct Contenido {
    pub titulo: String,
    pub descripcion: String,
    pub url_archivo: String,
    pub autenticado: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct HistorialEliminacion {
    pub titulo: String,
    pub descripcion: String,
    pub timestamp: u64,
}
#[derive(Clone)]
#[contracttype]
pub struct VersionContenido {
    pub titulo: String,
    pub descripcion: String,
    pub url_archivo: String,
    pub timestamp: u64,
}


#[contractimpl]
impl Contract {
    pub fn registrar_creador(env: Env, nombre: String, correo: String, direccion_wallet: String) -> Result<(), u32> {
        let key = symbol_short!("CREADORES");
        let mut creadores: Map<String, Creador> = env.storage().persistent().get(&key).unwrap_or_default();

        if creadores.contains_key(&direccion_wallet) {
            return Err(1); // ya registrado
        }

        let nuevo = Creador { username: nombre, email: correo, redes: None };
        creadores.set(direccion_wallet.clone(), nuevo);
        env.storage().persistent().set(&key, &creadores);

        Ok(())
    }

    pub fn eliminar_creador(env: Env, direccion_wallet: String, confirmar: bool) -> Result<(), u32> {
        let key = symbol_short!("CREADORES");
        let mut creadores: Map<String, Creador> = env.storage().persistent().get(&key).unwrap_or_default();

        if !confirmar {
            return Err(2); // no confirmado
        }

        if !creadores.contains_key(&direccion_wallet) {
            return Err(3); // no existe
        }

        creadores.remove(&direccion_wallet);
        env.storage().persistent().set(&key, &creadores);

        Ok(())
    }

    pub fn actualizar_perfil(env: Env, direccion_wallet: String, nuevo_nombre: String, nuevo_correo: String, nuevas_redes: Option<String>) -> Result<(), u32> {
        let key = symbol_short!("CREADORES");
        let mut creadores: Map<String, Creador> = env.storage().persistent().get(&key).unwrap_or_default();

        if !creadores.contains_key(&direccion_wallet) {
            return Err(4);
        }

        let actualizado = Creador {
            username: nuevo_nombre,
            email: nuevo_correo,
            redes: nuevas_redes,
        };

        creadores.set(direccion_wallet.clone(), actualizado);
        env.storage().persistent().set(&key, &creadores);

        Ok(())
    }

    pub fn consultar_creador(env: Env, direccion_wallet: String) -> Option<Creador> {
        let key = symbol_short!("CREADORES");
        let creadores: Map<String, Creador> = env.storage().persistent().get(&key).unwrap_or_default();
        creadores.get(direccion_wallet)
    }

    pub fn alta_contenido(env: Env, direccion_wallet: String, titulo: String, descripcion: String, url_archivo: String) -> Result<(), u32> {
        let key_creadores = symbol_short!("CREADORES");
        let key_contenido = symbol_short!("CONTENIDO");

        let creadores: Map<String, Creador> = env.storage().persistent().get(&key_creadores).unwrap_or_default();
        if !creadores.contains_key(&direccion_wallet) {
            return Err(5);
        }

        let autenticado = url_archivo.len() > 10;
        if !autenticado {
            return Err(6);
        }

        let nuevo = Contenido {
            titulo,
            descripcion,
            url_archivo,
            autenticado,
        };

        let mut contenidos: Map<String, Vec<Contenido>> = env.storage().persistent().get(&key_contenido).unwrap_or_default();
        let mut lista = contenidos.get(direccion_wallet.clone()).unwrap_or(Vec::new(&env));
        lista.push_back(nuevo);
        contenidos.set(direccion_wallet, lista);
        env.storage().persistent().set(&key_contenido, &contenidos);

        Ok(())
    }

    pub fn baja_contenido(env: Env, direccion_wallet: String, titulo: String, confirmar: bool) -> Result<(), u32> {
        let key_contenido = symbol_short!("CONTENIDO");
        let key_historial = symbol_short!("HISTORIAL");

        if !confirmar {
            return Err(7); // no confirmado
        }

        let mut contenidos: Map<String, Vec<Contenido>> = env.storage().persistent().get(&key_contenido).unwrap_or_default();
        let mut lista = contenidos.get(direccion_wallet.clone()).unwrap_or(Vec::new(&env));

        let index = lista.iter().position(|c| c.titulo == titulo);

        if let Some(i) = index {
            let contenido_eliminado = lista.get(i).unwrap();
            let historial = HistorialEliminacion {
                titulo: contenido_eliminado.titulo.clone(),
                descripcion: contenido_eliminado.descripcion.clone(),
                timestamp: env.ledger().timestamp(),
            };

            // Guardar historial
            let mut historiales: Map<String, Vec<HistorialEliminacion>> = env.storage().persistent().get(&key_historial).unwrap_or_default();
            let mut registros = historiales.get(direccion_wallet.clone()).unwrap_or(Vec::new(&env));
            registros.push_back(historial);
            historiales.set(direccion_wallet.clone(), registros);
            env.storage().persistent().set(&key_historial, &historiales);

            // Eliminar el contenido
            lista.remove(i);
            contenidos.set(direccion_wallet, lista);
            env.storage().persistent().set(&key_contenido, &contenidos);

            Ok(())
        } else {
            Err(8) // contenido no encontrado
        }
    }
    pub fn modificar_contenido(
        env: Env,
        direccion_wallet: String,
        titulo_antiguo: String,
        nuevo_titulo: String,
        nueva_descripcion: String,
        nueva_url: String,
    ) -> Result<(), u32> {
        let key_contenido = symbol_short!("CONTENIDO");
        let key_versiones = symbol_short!("VERSIONES");
    
        let mut contenidos: Map<String, Vec<Contenido>> = env.storage().persistent().get(&key_contenido).unwrap_or_default();
        let mut lista = contenidos.get(direccion_wallet.clone()).unwrap_or(Vec::new(&env));
    
        let index = lista.iter().position(|c| c.titulo == titulo_antiguo);
    
        if let Some(i) = index {
            let contenido_anterior = lista.get(i).unwrap();
    
            // Guardar versión previa
            let version = VersionContenido {
                titulo: contenido_anterior.titulo.clone(),
                descripcion: contenido_anterior.descripcion.clone(),
                url_archivo: contenido_anterior.url_archivo.clone(),
                timestamp: env.ledger().timestamp(),
            };
    
            let mut versiones: Map<String, Vec<VersionContenido>> = env.storage().persistent().get(&key_versiones).unwrap_or_default();
            let mut historial = versiones.get(direccion_wallet.clone()).unwrap_or(Vec::new(&env));
            historial.push_back(version);
            versiones.set(direccion_wallet.clone(), historial);
            env.storage().persistent().set(&key_versiones, &versiones);
    
            // Validar autenticidad del nuevo archivo
            let autenticado = nueva_url.len() > 10;
            if !autenticado {
                return Err(9); // URL no válida
            }
    
            // Actualizar contenido
            let actualizado = Contenido {
                titulo: nuevo_titulo,
                descripcion: nueva_descripcion,
                url_archivo: nueva_url,
                autenticado,
            };
    
            lista.set(i, actualizado);
            contenidos.set(direccion_wallet, lista);
            env.storage().persistent().set(&key_contenido, &contenidos);
    
            Ok(())
        } else {
            Err(10) // Contenido no encontrado
        }
    }
    
    pub fn consultar_contenido(env: Env, criterio: String) -> Vec<Contenido> {
        let key_contenido = symbol_short!("CONTENIDO");
        let contenidos: Map<String, Vec<Contenido>> = env.storage().persistent().get(&key_contenido).unwrap_or_default();
        let mut resultados = Vec::new(&env);
    
        for (_wallet, lista_contenido) in contenidos.iter() {
            for contenido in lista_contenido {
                if contenido.titulo.to_lowercase().contains(&criterio.to_lowercase())
                    || contenido.descripcion.to_lowercase().contains(&criterio.to_lowercase())
                {
                    resultados.push_back(contenido);
                }
            }
        }
    
        resultados
    }
    pub fn revisar_contenido(env: Env, contenido: Contenido) -> bool {
        let politicas_prohibidas = vec![
            String::from_str(&env, "pornografía"),
            String::from_str(&env, "violencia"),
            String::from_str(&env, "odio"),
        ];
    
        let descripcion = contenido.descripcion.to_lowercase();
    
        for palabra in politicas_prohibidas.iter() {
            if descripcion.contains(&palabra) {
                return false; // Contenido rechazado
            }
        }
    
        true // Contenido aprobado
    }
    pub fn compensar_creador(
        env: Env,
        consumidor: Address,
        creador: Address,
        cantidad_tokens: i128,
    ) -> Result<(), String> {
        let token_client = TokenClient::new(&env, &env.current_contract_address());
    
        if token_client.balance(&consumidor) < cantidad_tokens {
            return Err("Fondos insuficientes".into());
        }
    
        // Transferencia de tokens del consumidor al creador
        token_client.transfer(&consumidor, &creador, &cantidad_tokens);
    
        Ok(())
    }
    
    pub fn realizar_pago_tokens(
        env: Env,
        consumidor: Address,
        creador: Address,
        cantidad_tokens: i128,
    ) -> Result<(), String> {
        let blockchain_client = BlockchainClient::new(&env);
        let token_client = TokenClient::new(&env, &env.current_contract_address());
    
        // Paso 1: Consultar el saldo de tokens del consumidor
        let saldo_consumidor = token_client.balance(&consumidor);
    
        if saldo_consumidor < cantidad_tokens {
            return Err("Fondos insuficientes en la blockchain".into());
        }
    
        // Paso 2: Realizar la transacción en la blockchain
        // Transacción de tokens desde el consumidor al creador
        token_client.transfer(&consumidor, &creador, &cantidad_tokens);
    
        // Paso 3: Registrar la transacción en la blockchain
        blockchain_client.registrar_transaccion(
            &consumidor,
            &creador,
            cantidad_tokens,
            "Pago por contenido consumido",
        );
    
        Ok(())
    }
    
    pub fn validar_autenticidad_contenido(
        env: Env,
        contenido_id: String,
        creador: Address,
    ) -> Result<(), String> {
        let metadata_client = MetadataClient::new(&env);
        let verificador_originalidad = OriginalityVerifier::new(&env);
    
        // Paso 1: Verificar si el contenido ha sido subido correctamente
        let contenido = metadata_client.get_contenido(&contenido_id);
    
        if contenido.is_none() {
            return Err("Contenido no encontrado".into());
        }
    
        // Paso 2: Verificar la autenticidad del contenido
        let es_original = verificador_originalidad.verificar(&contenido.unwrap().data);
    
        if es_original {
            // Paso 3: Aceptar el contenido si es original
            metadata_client.marcar_como_valido(&contenido_id);
            Ok(())
        } else {
            // Si no es original, rechazar el contenido
            metadata_client.rechazar_contenido(&contenido_id);
            Err("Contenido no válido. No es original.".into())
        }
    }
    
    
}

mod test;
