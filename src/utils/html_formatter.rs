use std::fs::File;
use std::io::Write;

use crate::utils::lexer::Token;
use crate::utils::lexer::TokenKind;

fn span_generator(token: &Token, id: usize) -> String {
    let mut span = String::new();
    if token.kind == TokenKind::LineBreak {
        span.push_str("<br/>");
    } else if token.kind == TokenKind::Comment {
        span.push_str(&format!("<span id='span{}' class='{}'>{}</span><br/>", id, get_class_name(&token), token.value.clone()));
    } else {
        span.push_str(&format!("<span id='span{}' class='{}'>{}</span>", id, get_class_name(&token), token.value.clone() + " "));
    }
    span
}

fn tr_generator(token: &Token, id: usize) -> Option<String> {
    let mut tr = String::new();
    if token.kind == TokenKind::LineBreak {
        return None;
    }
    tr.push_str(&format!("<tr key='{}'>", id));
    tr.push_str(&format!("<td>{}</td>", token.value));
    tr.push_str(&format!("<td>{}</td>", get_class_name_spanish(&token)));
    tr.push_str("</tr>");
    Some(tr)
}

fn get_class_name(token: &Token) -> String {
    let token_kind = match token.kind {
        TokenKind::Integer => "integer",
        TokenKind::Float => "float",
        TokenKind::Assign => "operator",
        TokenKind::Plus => "operator",
        TokenKind::Minus => "operator",
        TokenKind::Asterisk => "operator",
        TokenKind::Slash => "operator",
        TokenKind::Percent => "operator",
        TokenKind::Power => "operator",
        TokenKind::LParen => "special-symbol",
        TokenKind::RParen => "special-symbol",
        TokenKind::Identifier => "identifier",
        TokenKind::Comment => "comment",
        TokenKind::Spaces => "spaces",
        TokenKind::SemiColon => "semicolon",
        TokenKind::LineBreak => "linebreak",
    };
    String::from(token_kind)
}

fn get_class_name_spanish(token: &Token) -> String {
    let token_kind = match token.kind {
        TokenKind::Integer => "Entero",
        TokenKind::Float => "Flotante",
        TokenKind::Assign => "Asignación",
        TokenKind::Plus => "Suma",
        TokenKind::Minus => "Resta",
        TokenKind::Asterisk => "Multiplicación",
        TokenKind::Slash => "División",
        TokenKind::Percent => "Módulo",
        TokenKind::Power => "Potencia",
        TokenKind::LParen => "Paréntesis de apertura",
        TokenKind::RParen => "Paréntesis de cierre",
        TokenKind::Identifier => "Identificador",
        TokenKind::Comment => "Comentario",
        TokenKind::Spaces => "Espacios",
        TokenKind::SemiColon => "Punto y coma",
        TokenKind::LineBreak => "Salto de línea",
    };
    String::from(token_kind)
}

pub fn get_html(tokens: &Vec<Vec<Token>>) {
    let mut spans: Vec<String> = Vec::new();
    let mut trs: Vec<String> = Vec::new();

    for line in tokens {
        for (i, token) in line.iter().enumerate() {
            spans.push(span_generator(&token, i));
            if let Some(tr) = tr_generator(&token, i) {
                trs.push(tr);
            }
        }
    }
    //make html file
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />

    <title>Lexer Analyzer</title>
    <meta name=\"description\" content=\"LexerOutput\" />
    <meta name=\"author\" content=\"AntonioNogueron\" />

    <link rel=\"stylesheet\" href=\"styles.css\" />
  </head>

  <body>
    <h1 class=\"h1-title\">Highlited Code</h1>
    <div class=\"code-container\">
      <code class=\"code-tag\">");

    for span in spans {
        html.push_str(&span);
    }

    html.push_str("</code>
    </div>
    <div class=\"dashboardContainer\">
      <div class=\"dashboardBottom\">
        <table>
          <thead>
            <tr>
              <th>Value</th>
              <th>Token</th>
            </tr>
          </thead>
          <tbody>");
    for tr in trs {
        html.push_str(&tr);
    }
    html.push_str("</tbody>
        </table>
      </div>
    </div>
  </body>
</html>");
    let mut file = File::create("output.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
}
