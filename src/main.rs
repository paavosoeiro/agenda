use agenda::{
    agenda::{Agenda, Contato},
    ui,
};

fn main() {
    let mut agenda = Agenda::new();

    loop {
        let opcao = ui::principal();
        if opcao == 1 {
            println!("Insira o nome do contato:");
            let mut nome = String::new();
            std::io::stdin()
                .read_line(&mut nome)
                .expect("Erro ao ler nome");
            let nome = nome.trim().to_string();
            println!("Insira o numero do contato:");
            let mut numero = String::new();
            std::io::stdin()
                .read_line(&mut numero)
                .expect("Erro ao ler numero");
            let numero = numero.trim().to_string();
            let contato = Contato::build(nome, numero);
            agenda.adicionar_contato(contato);

            std::process::Command::new("cmd")
                .args(["/c", "cls"])
                .spawn()
                .expect("cls command failed to start")
                .wait()
                .expect("cls command failed to run");
        } else if opcao == 2 {
            println!("Insira o nome do contato:");
            let mut nome = String::new();
            std::io::stdin()
                .read_line(&mut nome)
                .expect("Erro ao ler nome");
            let nome = nome.trim().to_string();
            let contato = agenda.buscar_contato(&nome);
            match contato {
                Some(c) => println!("Contato encontrado: {:?}", c),
                None => println!("Contato nao encontrado"),
            }
        } else if opcao == 3 {
            agenda.imprimir();
        }
        //
        else if opcao == 4 {
            break;
        }
    }
}
