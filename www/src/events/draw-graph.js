import $ from 'jquery';
import { distance, addVertex, addEdge, withOffset, fromDistance } from '../common/utils';
import { showToast } from '../components/toast/toast';
import { Colors } from '../common/constants';

export function drawGraph({ weighted, directed, asyclic }) {
    $('#plane').off();
    var { Graph, Point, Segment } = this;
    var lastp,
        prev,
        flag = false;

    function isValid(p) {
        let s = new Segment(lastp, p);
        for (let i = 0; i < Graph.totalSegments(); i++) {
            let si = Graph.segment(i);
            if (s.overlaps(si)) return false;
        }
        return true;
    }

    $('#plane').on('click', function (e) {
        e.preventDefault();
        let p = new Point(withOffset(e).x, withOffset(e).y);
        let np = Graph.totalPoints();
        if (np === 0) {
            addVertex(p, 'A');
            Graph.addPoint(p);
            return;
        }
        let k;
        for (k = 0; k < np; k++) {
            let q = Graph.point(k);
            let d = distance(p, q);
            if (d < 25) {
                p.x = q.x;
                p.y = q.y;
                break;
            }
        }
        if (flag) {
            $('.vrtx').eq(prev).attr('stroke', '#777');
            if (p.equals(lastp) || !isValid(p)) {
                $('line:last').remove();
                flag = false;
                return;
            }
            $('line:last').attr('x2', p.x);
            $('line:last').attr('y2', p.y);
            if (k === np) {
                if (np === 26) {
                    $('line:last').remove();
                    flag = false;
                    return;
                }
                addVertex(p, String.fromCharCode(65 + np));
                Graph.addPoint(p);
            }
            let s = new Segment(lastp, p);
            Graph.addSegment(s);
            weighted && addCost(p, lastp);
            if (directed) {
                if (asyclic && Graph.hasCycle()) {
                    showToast({
                        message: 'Please draw acyclic graph',
                        variant: 'error',
                    });
                    $('line:last').remove();
                    Graph.removeSegment(s);
                    flag = false;
                    return;
                }
                let q = fromDistance(lastp, p, 23);
                $('line:last').attr('x2', q.x);
                $('line:last').attr('y2', q.y);
            }
            flag = false;
        } else {
            if (k === np) {
                if (np < 26) {
                    addVertex(p, String.fromCharCode(65 + np));
                    Graph.addPoint(p);
                }
            } else {
                addEdge(p, p);
                $('.vrtx').eq(k).attr('stroke', Colors.visited);
                if (directed) {
                    $('line:last').attr('marker-end', 'url(#arrow)');
                }
                lastp = p;
                prev = k;
                flag = true;
            }
        }
    });

    $('#plane').on('mousemove', function (e) {
        e.preventDefault();
        if (flag) {
            let p = new Point(withOffset(e).x, withOffset(e).y);
            $('line:last').attr('x2', p.x);
            $('line:last').attr('y2', p.y);
        }
    });

    $('#plane').on('mouseleave', function (e) {
        e.preventDefault();
        if (flag) {
            $('line:last').remove();
            $('.vrtx').eq(prev).attr('stroke', '#777');
            flag = false;
        }
    });
}

function addCost(p, q) {
    let element = `
        <foreignObject width="30" height="30" x="${(p.x + q.x) / 2}" y="${(p.y + q.y) / 2}">
            <p class="cost" onclick="this.focus();event.stopPropagation();" contenteditable="true">
                ${Math.round(distance(p, q) / 20)}
            </p>
        </foreignObject>`;
    document.querySelector('#plane').innerHTML += element;
}