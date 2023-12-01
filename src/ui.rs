pub fn principal() -> u32 {
    println!("Agenda de contatos");
    println!("Insira a opcao desejada:");
    println!("1 - Adicionar contato");
    println!("2 - Buscar contato");
    println!("3 - Imprimir agenda");
    println!("4 - Sair");

    let mut opcao = String::new();

    std::io::stdin()
        .read_line(&mut opcao)
        .expect("Erro ao ler opcao");
    let opcao = opcao.trim().parse::<u32>().unwrap();
    opcao
}

