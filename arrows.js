/*"var shapes = [{orig: 'd5', dest: 'e4', brush: 'red'}, {orig: 'd4', brush: 'blue'}];"
"ground.setShapes(shapes);"*/
function drawArrows(from, to) {
    function mapper(e) {
        var o = {orig: from, brush: 'blue'};
        if(e) {
            o.dest = e;
        }
        return o;
    }
    var shapes = to.map(mapper);
    ground.setShapes(shapes);
}

function clearArrows() { ground.setShapes([]); }
