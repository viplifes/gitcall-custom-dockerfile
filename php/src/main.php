#!/usr/bin/env php
<?php

declare(strict_types=1);

require dirname(__DIR__) . "/vendor/autoload.php";
require __DIR__ . '/handler.php';

use Amp\ByteStream;
use Amp\Http\HttpStatus;
use Amp\Http\Server\DefaultErrorHandler;
use Amp\Http\Server\Driver\SocketClientFactory;
use Amp\Http\Server\Request;
use Amp\Http\Server\RequestHandler\ClosureRequestHandler;
use Amp\Http\Server\Response;
use Amp\Http\Server\SocketHttpServer;
use Amp\Log\StreamHandler;
use Amp\Socket;
use Monolog\Logger;
use function Amp\trapSignal;
use Psr\Log\LogLevel;

if (false === $uri = getenv('USERCODE_PROXY_ADDR')) {
    die("USERCODE_PROXY_ADDR env is required but not set");
}

$logHandler = new StreamHandler(ByteStream\getStdout(), LogLevel::INFO);
$logger = new Logger('server');
$logger->pushHandler($logHandler);
$logger->useLoggingLoopDetection(false);

$server = new SocketHttpServer(
    $logger,
    new Socket\ResourceServerSocketFactory(),
    new SocketClientFactory($logger),
    [],
    ["POST"],
);

$server->expose($uri);
$server->start(new ClosureRequestHandler(function (Request $request) use ($html): Response {
    $body = $request->getBody()->buffer();
    $data = json_decode($body, true);

    $jsonrpc =  $data["jsonrpc"];
    $id =  $data["id"];
    $params =  $data["params"];

    try {
        $reslut = handle($id, $params);
        return new Response(
            status: HttpStatus::OK,
            headers: ["content-type" => "application/json; charset=utf-8"],
            body: json_encode([
                'jsonrpc' => $jsonrpc,
                'id' => $id,
                'result' => $reslut
            ]),
        );
    } catch (\Throwable $e) {
        return new Response(
            status: HttpStatus::OK,
            headers: ["content-type" => "application/json; charset=utf-8"],
            body: json_encode([
                'jsonrpc' => $jsonrpc,
                'id' => $id,
                'code' => 1,
                'message' => $e->getMessage()
            ]),
        );
    }
}), new DefaultErrorHandler());


// Await a termination signal to be received.
$signal = trapSignal([\SIGHUP, \SIGINT, \SIGQUIT, \SIGTERM]);
$logger->info(sprintf("Received signal %d, stopping HTTP server", $signal));
$server->stop();
