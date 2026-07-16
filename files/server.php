<?php
header("Access-Control-Allow-Origin: *");
// header("Access-Control-Allow-Methods: GET, POST, OPTIONS");
// header("Access-Control-Allow-Headers: Content-Type");
header('Content-Type: application/json; charset=utf-8');
$form = file_get_contents('./form.json');
echo $form;
die();


// "backup": {
//     "when": "tomorrow",
//     "where": "there"
// }


// ,
// "tasks": [
//     {"task_name" : "First", "priority": "h"},
//     {"task_name" : "Second", "priority": "h"},
//     {"task_name" : "Third" }
// ]

// {
//     "name": "state",
//     "type": "select",
//     "label": "State",
//     "default": "MA",
//     "classes":["col-6"],
//     "options": [
//         {"value": "MA", "label": "Massachusetts"},
//         {"value": "OH", "label": "Ohio"}
//     ]

// },

// {
//     "name": "city",
//     "type": "text",
//     "label": "City",
//     "default": "Gothem",
//     "classes":["col-6"]
// },
// {
//     "name": "state",
//     "type": "text",
//     "label": "State",
//     "default": "MA",
//     "classes":["col-6"]
// },
