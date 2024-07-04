fn pesquisa_binaria(vetor: &Vec<usize>, acerto: usize) -> Option<usize> {
    let mut baixo: usize = 0;
    let mut alto: usize = vetor.len() - 1;

    while baixo <= alto {
        let meio: usize = (baixo + alto) / 2 as usize;
        let chute: usize = vetor[meio];
        if chute == acerto {
            println!("Acertou! O número era {acerto}");
            return Some(meio);
        } else if chute > acerto {
            println!("Muito alto");
            alto = meio - 1
        } else {
            println!("Muito baixo");
            baixo = meio + 1;
        } 
    }
    None
}

fn busca_menor(vetor: &Vec<i32>) -> usize {
    // Pega o menor item do vetor e o menor indice
    let mut menor = vetor[0];
    let mut menor_indice = 0;

    // faz um forloop no array pegando o index e o item
    for (index, _item) in vetor.iter().enumerate() {
        // se o index for correspontente ao primeiro item, pule-o
        if index == 0 {
            continue;
        } else {
            // se o item atual for menor do que o item anterior, 
            // atualize o menor e o indice dele
            if vetor[index] < menor {
                menor = vetor[index];
                menor_indice = index;
            }
        }
    }
    // retorne o menor indice
    menor_indice
    // Esse algoritmo possui tempo O(n) pois precisa percorrer todo o array
    // para achar o menor.
}

fn ordenacao_selecao(mut vetor: Vec<i32>) -> Vec<i32> {
    // inicializa o novo array que terá todos os itens ordenados
    let mut novo_arr: Vec<i32> = Vec::new();
    // loop no vetor inicial
    for _ in vetor.clone().into_iter() {
        // acha o menor numero do vetor
        let menor = busca_menor(&vetor);
        // coloca o menor numero no novo array
        novo_arr.push(vetor.remove(menor));
    }
    // retorna o novo array ordenado
    novo_arr
    // Esse algoritmo possui tempo O(n²) pois faz uma busca menor para cada 
    // vez que um elemento eh colocado dentro do novo array (O(n * n));
}

fn main() {
    let minha_lista: Vec<usize> = Vec::from([1, 3, 5, 7, 9]);
    match pesquisa_binaria(&minha_lista, 3) {
        Some(x) => println!("Result {x}"),
        None => println!("Not in the Vec"),
    }
    println!();
    match pesquisa_binaria(&minha_lista, 10) {
        Some(x) => println!("Result {x}"),
        None => println!("Not in vec"),
    }

    println!("{:?}", ordenacao_selecao(Vec::from([5, 3, 6, 2, 10])));
}
