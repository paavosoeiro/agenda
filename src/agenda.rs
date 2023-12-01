#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Contato {
    nome: String,
    numero: String,
    email: String,
}

impl Contato {
    pub fn build(nome: String, numero: String) -> Self {
        Contato {
            nome,
            numero,
            ..Default::default()
        }
    }

    pub fn atualizar_contato(&mut self, numero: String) {
        self.numero = numero;
    }
}

// impl Default for Contato {
//     fn default() -> Self {
//         Contato {
//             nome: "".to_string(),
//             numero: "".to_string(),
//             email: "".to_string(),
//         }
//     }
// }

#[derive(Debug, Default)]
pub struct Agenda {
    contatos: Vec<Contato>,
}

impl Agenda {
    pub fn new() -> Self {
        Agenda { contatos: vec![] }
    }

    pub fn adicionar_contato(&mut self, contato: Contato) {
        self.contatos.push(contato);
    }

    pub fn buscar_contato(&mut self, nome: &str) -> Option<&mut Contato> {
        self.contatos.iter_mut().find(|c| c.nome == nome)
    }

    pub fn imprimir(&self) {
        for contato in self.contatos.iter() {
            println!("Nome: {} - Telefone: {}", contato.nome, contato.numero);
        }
    }
}

