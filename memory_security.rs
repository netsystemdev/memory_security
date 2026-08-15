


 

  // ** Estudo : Gerenciamento de memoria Ativa ** 


  // ____ [ Arc mutex + Box ] ________ 


 


   use std::sync::{Arc , Mutex}; 

   use std::thread;       // process thread library     


   use std::io; 

   use std::io::prelude::*;       // > importacao de toda a biblioteca  





 type VectorAsyncException<T> = Result<T , Box<dyn std::error::Error>>;

  // ............. < interface > ............... 


  pub trait AsyncVectorOperations {




            // insere o chamado no vector 


            fn insert_new_chamado(&mut self , chamado : Chamado) -> VectorAsyncException<()> ; 


            // ** confere se o chamado informado existe ** // 


            fn search_by_index_chamado(&self , index_chamado : i128) -> VectorAsyncException<Vec<Chamado>> ;        // < assim , retorna os valores do chamado em aberto ! > 


            fn query_all_results(&self) -> VectorAsyncException<Vec<Chamado>> ;  



            // * funcao que printa na tela os resultados */ 


            fn relatorio_by_results(&self) -> VectorAsyncException<()> ;

 

  }


 



   // .............. < oop > .................. 


   #[derive(Debug , Clone)] 

   pub struct Chamado             // ITSM Chamado + ocorrência 

   {



   	        serial : i128 ,     // protocolo do chamado 


   	        data   : String ,    // Data do chamado 


   	        solicitante : String , 


              // < itens 'pub' , que podem ser modificados na Arc<mutex> // 
 
   	         pub descricao : String , 


   	         pub  priority : i32 ,      // Classificacao : 1 , 2 ou 3 ; 


   	         pub prazo : i32 ,          // prazo : horas 




   }



   //  ........... >> constructor >>  ................ 

   impl Chamado {



   		       pub fn new(serial : i128 ,  

   		       		  data : impl Into<String> , 

                      solicitante : impl Into<String> , 

   		       		  descricao : impl Into<String> , 

   		       		  priority : i32, 

   		       		  prazo :  i32) -> Self 

   		       {


   		       		  Self {


   		       		  			serial , 


   		       		  			// _ impl _ 

   		       		  			data : data.into() , 

                                solicitante : solicitante.into() , 

   		       		  			descricao : descricao.into() , 

   		       		  			priority  , 

   		       		  			prazo ,     




   		       		  }



   		       }





   }    






 



   //  ............... < OOP > ........................ 


   // Associação da classe Chamado -> Analista TI // 


   #[derive(Debug , Clone)] 

   pub struct AnalistaTI 

   {


   		      // dados do Analista 


   		      matricula : i64 , 

   		      nome      : String , 

   		      gestor    : String , 

   		      cargo     : String ,   




   		      // *** Vector da lista de chamados *** 


   		      arraylist_chamado : Vec<Chamado> , 

   }
  


   // ................. constructor  ..................


   impl AnalistaTI 

   {                                                                                       


           // constructor 

           pub fn new(matricula : i64 , 

                  nome : impl Into<String> , 

                  gestor : impl Into<String> , 

                  cargo  : impl Into<String>) -> Self {            


                  // << chama >> 

                    

                        Self {


                                 matricula , 


                                 nome : nome.into() , 

                                 gestor : gestor.into() , 

                                 cargo  : cargo.into() , 



                                 // _ inicialização vazia e direta fora do new , mas apenas na referencia

                                 arraylist_chamado : Vec::new() ,       // inicialização , de forma 'limpa' , 



                        }



              }

    }   



 
    // ............ < vector operations > ........................ 

    impl AsyncVectorOperations for AnalistaTI 

    {



             fn insert_new_chamado(&mut self , chamado : Chamado) -> VectorAsyncException<()> 
            
             {  


                      
                            // adiciona a base de dados


                            self.arraylist_chamado.push(chamado);

                            
                           Ok(())     


             }  




             // ** consulta de dados no vector > informa se o chamado realmente existe ** / 


             fn search_by_index_chamado(&self , index_chamado : i128) -> VectorAsyncException<Vec<Chamado>> 

             {


                          // verifica se o seu vec_ esta vazio !


                        if self.arraylist_chamado.is_empty() {


                                // retorna erro interno 


                                return Err("VectorOperations.Exception : Vector vazio ! insira elementos ou chame o suporte ! ".into()); 



                        }     





                      let mut resultados: Vec<Chamado> = self 

                            .arraylist_chamado 

                            .iter() 


                            // * dentro do filter , criamos uma funcao de iteracao + clousure * 

                            .filter(|chamado| chamado.serial == index_chamado) 

                            .cloned() 

                            .collect(); 





                      // retorna a lista 


                    Ok(resultados)       // -----> retorna a base formatada

             }





             // ** retorna todos os elementos ** 


             fn query_all_results(&self) -> VectorAsyncException<Vec<Chamado>> 

             {


                        // verifica se o seu vec_ esta vazio !


                        if self.arraylist_chamado.is_empty() {


                                // retorna erro interno 


                                return Err("VectorOperations.Exception : Vector vazio ! insira elementos ou chame o suporte ! ".into()); 



                        }  




                        Ok(self.arraylist_chamado.clone())       // retorna o vec e nao , a mensagem formatada dos registros 



             }





             // ** funcao extra  :>  devolve a variavel formatada 


            // * funcao que printa na tela os resultados */ 


            fn relatorio_by_results(&self) -> VectorAsyncException<()> 

            {




                            // iterador para processo de formatacao 


                          println!(" ================ RESULTADOS ===================== "); 
    


                            for it in self.arraylist_chamado.clone() 

                            {


                                  
                                    println!("\n"); 

                                    println!(" SERIAL CHAMADO  : {} " , it.serial);

                                    println!(" DATA CHAMADO    : {} " , it.data); 

                                    println!(" SOLICITANTE     : {} " , it.solicitante.to_uppercase()); 

                                    println!(" DESCRICAO       : {} " , it.descricao.to_uppercase()); 

                                    println!(" Priority        : {} " , it.priority); 

                                    println!(" TEMPO SLA / horas  : {} " , it.prazo);



                            }




                             println!("\n"); 



                  Ok(())
          


            }




    }













   //  ............... < Entity Class >  ....................... 


              // Arc Mutex > Gerencia TI Process // 


       #[derive(Debug , Clone)]  

       pub struct GerenciaChamadosSLA {



                  // _ < Arc Security > _ 

                   security_mutex_chamado : Arc<Mutex<Chamado>> ,    // > _ alteração na memoria -> chamado 



       }      





       // ........................ constructor  ...................... 

       impl GerenciaChamadosSLA {


                pub fn new(security_mutex_chamado : Arc<Mutex<Chamado>>) -> Self  

                {


                        Self {


                                 security_mutex_chamado : security_mutex_chamado,


                        }


                }








                 // .. < metodo de seguranca , para alteracao da chave do registro > + 'locked' () 
        


                 // _ * Metodo de segurança + lock() , ao qual destrava a alteracao do registro , no processo de modificacao na memoria do Chamado  


                 pub fn arc_mutex_security_update_chamado(&self , 

                        // * classificam-se 3 novas variaveis : 

                        nova_descricao : String , 

                        nova_prioridade : i32 , 

                        novo_prazo : i32) -> TRuntime<()> {



                        // _ 'lock'() , em que permite abrir a alteracao 


                        let mut mutex_key = self 

                                .security_mutex_chamado 

                                .lock()              // < !! _ permite a alteracao _ !! > 

                                .map_err(|_| "MutexSecurityAcess.Error :: Falha no processo de segurança , para alteração do chamado !")?; 




                        // _ classificam-se os 3 atributos alterados os seus registros com segurança 


                        
                        mutex_key.descricao = nova_descricao; 

                        mutex_key.priority = nova_prioridade; 

                        mutex_key.prazo = novo_prazo; 




                        Ok(())

                 }


       }






      













  type TRuntime<T> = Result<T , Box<dyn std::error::Error>>; 

  // .............. < _ main class _ > ........................ 

  fn main() -> TRuntime<()> 

  {


             // << Técnico responsável >> 


        let mut joao_ribeiro = AnalistaTI::new(35656 , 

                "Joao ribeiro das neves",  

                "Eduardo Gonçalves Cienciano",  

                "Tec. Service Desk - N1", 


        );


                
   
    

    joao_ribeiro.insert_new_chamado(



        // < _ instance class _ > 

          Chamado::new(


                202600933655665 , 

                "10/08/2026" , 

                "Rogerio Ribeiro Alves - Serv. Publico TJRJ", 




                "Entrega de Notebook - Enc. Setor técnico service N2 Campo (Efetuar agendamento)" , 

                2 , 

                3 ,    // equivale o numero de horas para resolução


        ))?;




   joao_ribeiro.insert_new_chamado( 

          Chamado::new(  

                202600933988984 , 

                "10/08/2026" , 

                "Rogerio Ribeiro Alves - Serv. Publico TJRJ", 




                "Concessao de Acesso - Sistema SEI e Integra Judicial - Plantonista MPRJ", 

                2 , 

                1 ,      // 1h para acesso ao sistema - login e senha 

                
        ))?;




    joao_ribeiro.insert_new_chamado(

             Chamado::new(


                202600135665565, 

                "11/08/2026" ,  

                "Dr. Paulo Ribeiro Rodrigues - PROM. Justiça - MPRJ", 




                "Registro de Evento dos Magistrados - marcação para o Setor GMI Avançado" ,

                1 ,     // Priority máxima 

                6 ,    // 06hs ainda antes do evento da magistratura

                
            ))?; 



 

        let _ = joao_ribeiro.relatorio_by_results()?; 






        // << Alteracao de segurança : promotor deseja alterar o evento para 12 horas (evento demarcate) : 


        // < instancia da gerencia de TI > 



        /*

               *Qual o n do chamado ? 

               * descricao 

               * prioridade e prazo

               * A Quem se refere a alteracao dos registros na memoria ? 


        */



        // 1. Search : buscamos pelo chamado ao qual devemos alterar


        let chamado_consulta = joao_ribeiro.search_by_index_chamado(202600135665565)?;     // pesquisa a ID serial




        // if let some ==> referencia somente ao resultado programado , para acionar o protocolo de alteração 


        if let Some(chamado_service) = chamado_consulta.first() 

        {


                // ** referencia de chamado ao Arc<Mutex<Class>> 

                let chamado_compartilhado = Arc::new(Mutex::new(chamado_service.clone())); 



                // Chama a Instancia da GerenciaSLA -> Coordenador / Gerente De TI 

                let gerencia_coordenacao = GerenciaChamadosSLA::new(

                        chamado_compartilhado.clone() 

                );



               
                // segmento de alteracao , 


                gerencia_coordenacao.arc_mutex_security_update_chamado(


                        // Alteracoes 

                        "Alteração : Evento do Diario Oficial alterado para 12hs o chamado de entrega de convites".to_string() , 

                        1 , 

                        12   // de 6 para 12hs ,


                )?;   // 



                // ------------ <> exibicao dos resultados <>  -------------------------- 


                println!(" ============= <> ALTERACAO DO REGISTRO - CHAMADO  <>  ====================== "); 

                let update_mutex_security = chamado_compartilhado.lock().unwrap(); 


                println!("\n"); 


                println!(" DESCRICAO - ACOMPANHAMENTO   :   {} " , update_mutex_security.descricao); 

                println!(" N PRIORITY - ACOMPANHAMENTO   :   {} " , update_mutex_security.priority); 

                println!(" TEMPLO SLA/ Horas  - ACOMPANHAMENTO   :   {} " , update_mutex_security.prazo); 






        }





        let _ = io::stdout().flush().unwrap(); 
   
        let _ = io::stdin().read(&mut [0u8]).unwrap();  // liberacao + tecla _ ANSI
  		


        Ok(())
  }
