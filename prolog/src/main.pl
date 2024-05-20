:- use_module(library(http/http_server)).
:- use_module(library(http/http_json)).
:- use_module(library(http/json_convert)).
:- use_module(usercode).

:- initialization
    get_port(Port),
    http_server([port(Port)]).

:- http_handler(root(.), request, []).

get_port(Port) :-
  (getenv('GITCALL_PORT', PortStr) ->  
    atom_number(PortStr, Port); 
    format('GITCALL_PORT env is required but not set~n', []),
    halt(1)
  ).

request(Request) :-
  http_read_json_dict(Request, Json),
  catch(call_usercode(Json), Err, resp_error(Json, Err)).

call_usercode(Json) :-
  usercode:handle(Json.params, Result),
  reply_json_dict(json([jsonrpc=Json.jsonrpc, id=Json.id, result=Result])).

resp_error(Json, Err) :-
  print_message(error, Err),
  format(atom(Error), '~q', [Err]),
  reply_json(json([jsonrpc=Json.jsonrpc, id=Json.id, error=json([code=1, message=Error])])).
