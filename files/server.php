<?php
header("Access-Control-Allow-Origin: *");
// header("Access-Control-Allow-Methods: GET, POST, OPTIONS");
// header("Access-Control-Allow-Headers: Content-Type");
header('Content-Type: application/json; charset=utf-8');
$form = file_get_contents('./def2.json');
echo $form;
die();


// "backup": {
//     "when": "tomorrow",
//     "where": "there"
// }
