#[derive(Debug)]
struct Aluno {
    nome: String,
    notas: [f32; 3],
}

impl Aluno {
    fn calcula_media(&self) -> f32 {
        (self.notas[0] + self.notas[1] + self.notas[2]) / 3.0
    }

    fn aprovacao(&self) -> bool {
        self.calcula_media() >= 7.0
    }

    fn printa_boletim(&self) {
        println!("==============================");
        println!("{0}", self.nome);
        println!("------------------------------");
        println!("|  A1  |  A2  |  A3    |  M  |");
        println!(
            "| {:.2} | {:.2} | {:.2}   | {:.2}|",
            self.notas[0],
            self.notas[1],
            self.notas[2],
            self.calcula_media()
        );
        println!("------------------------------");
        println!(
            "Situação: {}",
            if self.aprovacao() {
                "Aprovado"
            } else {
                "Retido"
            }
        );
        println!("==============================");
    }
}

fn maior_media(a: &[Aluno]) -> Aluno {
    let mut maior = 0;
    for i in 1..a.len() {
        if a[i].calcula_media() > a[maior].calcula_media() {
            maior = i;
        }
    }

    Aluno {
        nome: a[maior].nome.clone(),
        notas: a[maior].notas,
    }
}

fn main() {}
