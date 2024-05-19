package com.corezoid.gitcall.runner;

import com.sun.net.httpserver.HttpServer;
import java.net.InetSocketAddress;
import java.util.Map;
import java.util.concurrent.Executors;

import com.google.gson.Gson;

import java.nio.charset.StandardCharsets;


public class Gitcall {

    public static void main(String[] args) throws Exception {
        String uri = System.getenv("USERCODE_PROXY_ADDR");
        if (uri == null || uri.isEmpty()) {
            System.out.println("USERCODE_PROXY_ADDR env is required but not set");
            throw new Exception("USERCODE_PROXY_ADDR env is required but not set");
        }

        Integer port = Integer.parseInt(uri.split(":")[1]);
        var gson = new Gson();
        var server = HttpServer.create(new InetSocketAddress(port), 0);
        server.setExecutor(Executors.newVirtualThreadPerTaskExecutor());
        server.createContext("/").setHandler(exchange -> {
            String strBody = new String(exchange.getRequestBody().readAllBytes(), StandardCharsets.UTF_8);        
            var request = gson.fromJson(strBody, JsonRpcRequest.class);
            var response = new JsonRpcResponse(request.jsonrpc, request.id);
            try {
                var data = Gitcall.handle(request.params);
                response.result = data;
            } catch (Exception e) {                
                response.error = new JsonRpcResponse.JsonRpcError(1, e.toString());
            } 
            var jsonRessp = gson.toJson(response).getBytes();
            exchange.sendResponseHeaders(200, jsonRessp.length);
            try (var os = exchange.getResponseBody()) {
                os.write(jsonRessp);
            }
        });
        server.start();
        System.out.println( String.format("server listen: %s", port));
    }


    public static Map<String, Object> handle(Map<String, Object> data) throws Exception {
       data.put("java", "Hello world!");
       return data;
    }

}
