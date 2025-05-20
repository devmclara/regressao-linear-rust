/// Módulo para análise de séries temporais com regressão linear simples.

/// Estrutura para armazenar os coeficientes da regressão linear.
#[derive(Debug)]
pub struct RegressaoLinear {
    pub intercepto: f64,
    pub coeficiente: f64,
}

impl RegressaoLinear {
    /// Calcula os coeficientes da regressão linear (intercepto e coeficiente angular)
    /// usando o método dos mínimos quadrados, para os dados (x, y).
    /// x é a sequência temporal (ex: 0,1,2,...), y são os valores da série.
    pub fn fit(x: &[f64], y: &[f64]) -> Result<Self, String> {
        if x.len() != y.len() || x.is_empty() {
            return Err("Vetores x e y devem ter o mesmo tamanho e não podem ser vazios".to_string());
        }

        let n = x.len() as f64;

        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();

        let sum_x2: f64 = x.iter().map(|v| v * v).sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();

        let denominador = n * sum_x2 - sum_x * sum_x;
        if denominador == 0.0 {
            return Err("Divisão por zero no cálculo dos coeficientes".to_string());
        }

        let coeficiente = (n * sum_xy - sum_x * sum_y) / denominador;
        let intercepto = (sum_y - coeficiente * sum_x) / n;

        Ok(RegressaoLinear {
            intercepto,
            coeficiente,
        })
    }

    /// Faz a previsão do valor para um dado x (período futuro)
    pub fn prever(&self, x: f64) -> f64 {
        self.intercepto + self.coeficiente * x
    }

    /// Calcula o coeficiente de determinação R².
    /// Mede o quão bem os dados se ajustam à regressão.
    pub fn r2(&self, x: &[f64], y: &[f64]) -> f64 {
        let media_y: f64 = y.iter().sum::<f64>() / y.len() as f64;

        let ss_tot: f64 = y.iter().map(|yi| (yi - media_y).powi(2)).sum();
        let ss_res: f64 = x.iter().zip(y.iter())
            .map(|(xi, yi)| {
                let y_pred = self.prever(*xi);
                (yi - y_pred).powi(2)
            })
            .sum();

        1.0 - (ss_res / ss_tot)
    }

    /// Calcula o erro quadrático médio (MSE).
    pub fn mse(&self, x: &[f64], y: &[f64]) -> f64 {
        let n = y.len() as f64;
        let erro_total: f64 = x.iter().zip(y.iter())
            .map(|(xi, yi)| {
                let y_pred = self.prever(*xi);
                (yi - y_pred).powi(2)
            })
            .sum();
        erro_total / n
    }
}

fn main() {
    // Exemplo simples de uso

    // Série temporal: valores de y para tempos x=0,1,2,3,4
    let y = [2.0, 3.0, 5.0, 7.0, 11.0];
    let x: Vec<f64> = (0..y.len()).map(|v| v as f64).collect();

    // Ajusta a regressão linear
    let modelo = RegressaoLinear::fit(&x, &y).expect("Erro no cálculo da regressão");

    println!("Modelo ajustado: {:?}", modelo);
    println!("Coeficiente de determinação R²: {:.4}", modelo.r2(&x, &y));
    println!("Erro quadrático médio (MSE): {:.4}", modelo.mse(&x, &y));

    // Previsão para os próximos 3 períodos
    for i in 5..8 {
        let pred = modelo.prever(i as f64);
        println!("Previsão para t = {}: {:.4}", i, pred);
    }
}
