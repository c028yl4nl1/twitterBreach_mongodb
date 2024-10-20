# OSINT Email Searcher twitterBreach

## Sobre

Durante meus estudos de OSINT , me deparei com um banco de dados vazado na internet "TwitterBreach" ![image](https://github.com/user-attachments/assets/2b10e084-aa5e-4197-8ed3-e3ef736f439a)
Decidi baixá-lo e criar este programa em Rust para facilitar a busca rápida de emails. A ideia é tornar a análise de dados mais eficiente.




- Um programa que se conecta a um banco de dados MongoDB.
- Uma maneira prática de buscar emails de forma rápida e eficiente.

## `ulimit`

Durante o desenvolvimento, enfrentei um problema : um erro relacionado ao limite de arquivos abertos, conhecido como "Too many open files" oh bgl chato ![image](https://github.com/user-attachments/assets/7fcf387f-dbed-4856-ad76-0cf38de85471)


1. **Configuração inicial**: Pensei que o problema estivesse no hd, então configurei o `ulimit` para o máximo usando `systemctl`. ![image](https://github.com/user-attachments/assets/838ffa84-8281-4c1c-92cc-78b0eac8fac0)

   
2. **Tentativas sem sucesso**: Apesar de ter feito tudo que estava ao meu alcance — desde explorar fóruns até buscar ajuda em IA — não consegui resolver a situação. O erro persistia, e eu estava começando a ficar com raiva kkk. 

3. **A solução**: Foi então que decidi implementar uma solução alternativa: aumentar o `ulimit` diretamente no processo do meu programa. Essa abordagem finalmente funcionou!
4.  ![image](https://github.com/user-attachments/assets/6295b948-6bca-4c09-9c0d-e3ad142f86f9)

5.   ![image](https://github.com/user-attachments/assets/bd0feaa5-71de-44d2-a167-49904e3c30b5)


## Como ficou ;

- use db.twitter200milhoes.createIndex({email: 1})

  
![image](https://github.com/user-attachments/assets/e6025385-2a2e-4621-a00f-4d415cc15ec2)

