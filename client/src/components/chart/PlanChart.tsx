import styles from "./PlanChart.module.scss"
import classNames from "classnames";
import * as d3 from "d3";
import {useEffect, useState} from "react";
import React from "react";
import { curveBasis } from "d3";
import {Recurring, RecurringApi} from "../../api";
import handleFetchError from "../../hooks/handleFetchError";

const cx = classNames.bind(styles);

type PlanChartProps = {

};

const recurringApi = new RecurringApi();

export function PlanChart(props: PlanChartProps) {
    const focusHeight = 100;
    const height = 440;
    const width = 1000;
    const margin = ({top: 20, right: 20, bottom: 30, left: 40});
    const predictionStart = new Date("2011-01-01");
    const [recurrings, setRecurrings] = useState<Recurring[]>();
    const [error, setError] = useState<string>();

    useEffect(() => {
        getRecurrings();
    }, []);

    async function getRecurrings() {
        try {
            const recurrings = await recurringApi.getRecurrings();
            setRecurrings(recurrings);
        } catch (e) {
            setError(await handleFetchError(e));
        }
    }

    useEffect(() => {
        if (recurrings)
        getData();
    }, [recurrings]);

    async function getData() {
        const d = await d3.csv("/data.csv");
        const data = Object.assign((d).map(({date, close}) =>
            ({date: new Date(date), value: parseFloat(close)})));

        const knownData = data.filter(f => f.date <= predictionStart);
        const predictedData = data.filter(f => f.date >= predictionStart);

        console.log(knownData, predictedData);



        const area = (x, y) => d3.area()
            .defined((d: any) => !isNaN(d.value))
            .x((d: any)  => x(d.date))
            .y0(y(0))
            .y1((d: any)  => y(d.value));

        const line = (x, y) => d3.area()
            .curve(curveBasis)
            .defined((d: any) => !isNaN(d.value))
            .x((d: any)  => x(d.date))
            .y((d: any)  => y(d.value));

        const x = d3.scaleUtc()
            .domain(d3.extent(data,  (d: any) => d.date) as any)
            .range([margin.left, width - margin.right]);

        const maxY = d3.max(data,  (d: any) => d.value);
        const y = d3.scaleLinear()
            .domain([0, maxY] as any)
            .range([height - margin.bottom, margin.top]);

        const xAxis = (g, x, height) => g
            .attr("transform", `translate(0,${height - margin.bottom})`)
            .call(d3.axisBottom(x).ticks(width / 80).tickSizeOuter(0));

        const yAxis = (g, y, title) => g
            .attr("transform", `translate(${margin.left},0)`)
            .call(d3.axisLeft(y))
            .call(g => g.select(".domain").remove())
            .call(g => g.selectAll(".title").data([title]).join("text")
                .attr("class", "title")
                .attr("x", -margin.left)
                .attr("y", 10)
                .attr("fill", "currentColor")
                .attr("text-anchor", "start")
                .text(title));

        function createChart() {
            const svg = d3.create("svg")
                .attr("class", styles.svg + " " + styles.chart)
                .attr("viewBox", [0, 0, width, height] as any)
                .style("display", "block");

            svg.append("linearGradient")
                .attr("id", "areaGradient")
                .attr("gradientUnits", "userSpaceOnUse")
                .attr("x1", 0).attr("y1", 0)
                .attr("x2", 0).attr("y2", maxY)
                .selectAll("stop")
                .data([
                    {offset: "0%", color: "#21c19c", opacity: 0},
                    {offset: "100%", color: "#21c19c", opacity: 1},
                ])
                .enter().append("stop")
                .attr("stop-opacity", function(d) { return d.opacity; })
                .attr("offset", function(d) { return d.offset; })
                .attr("stop-color", function(d) { return d.color; });

            const clipId = "clipPath";

            svg.append("clipPath")
                .attr("id", clipId)
                .append("rect")
                .attr("x", margin.left)
                .attr("y", 0)
                .attr("height", height)
                .attr("width", width - margin.left - margin.right);


            svg.append("clipPath")
                .attr("id", "rectClip")
                .append("rect")
                .attr("x", margin.left)
                .attr("y", 0)
                .attr("height", height)
                .attr("width", width - margin.left - margin.right);

            const gx = svg.append("g");

            const gy = svg.append("g");

            const knownPath = svg.append("path")
                .datum(knownData)
                .attr("clip-path", "url(#" + clipId + ")")
                .attr("class", styles.path + " " + styles["path--known"]);

            const knownArea = svg.append("path")
                .datum(knownData)
                .attr("clip-path", "url(#" + clipId + ")")
                .attr("class", styles.area);

            const predictedPath = svg.append("path")
                .datum(predictedData)
                .attr("clip-path", "url(#" + clipId + ")")
                .attr("class", styles.path + " " + styles["path--predicted"]);


            const node = svg.node();


            (node as any).update = function(focusX, focusY) {
                gx.call(xAxis, focusX, height);
                gy.call(yAxis, focusY, data.y);
                knownArea.attr("d", area(focusX, focusY) as any);
                knownPath.attr("d", line(focusX, focusY) as any);
                predictedPath.attr("d", line(focusX, focusY) as any);

                createRects(svg, focusX, false, height - margin.bottom);
            };
            return node;
        }

        const chart = createChart();

        function createFocus() {
            const svg = d3.create("svg")
                .attr("class", styles.svg + " " + styles.focus)
                .attr("viewBox", [0, 0, width, focusHeight] as any)
                .style("display", "block");

            const brush = d3.brushX()
                .extent([[margin.left, 0.5], [width - margin.right, focusHeight - margin.bottom + 0.5]])
                .on("brush", brushed)
                .on("end", brushended);

            const defaultSelection = [x(d3.utcYear.offset(x.domain()[1], -1)), x.range()[1]];

            svg.append("g")
                .call(xAxis, x, focusHeight);

            svg.append("path")
                .datum(knownData)
                .attr("d", line(x, y.copy().range([focusHeight - margin.bottom, 4])) as any)
                .attr("class", styles.path+ " " + styles["path--known"]);

            svg.append("path")
                .datum(predictedData)
                .attr("d", line(x, y.copy().range([focusHeight - margin.bottom, 4])) as any)
                .attr("class", styles.path + " " + styles["path--predicted"]);

            const gb = svg.append("g")
                .call(brush)
                .call(brush.move, defaultSelection);

            createRects(svg, x, true, focusHeight - 30);

            function brushed({selection}) {
                if (selection) {
                    svg.property("value", selection.map(x.invert, x).map(d3.utcDay.round));
                    svg.dispatch("input");
                    requestAnimationFrame(() => {
                        update();
                    });
                }
            }

            function brushended({selection}) {
                if (!selection) {
                    gb.call(brush.move, defaultSelection);
                }
            }

            return svg.node();
        }

        const focus = createFocus();

        function update() {
            const [minX, maxX] = (focus as any).value as any;
            const maxY = d3.max(data, (d: any) => minX <= d.date && d.date <= maxX ? d.value as any : NaN);
            (chart as any).update(x.copy().domain((focus as any).value as any), y.copy().domain([0, maxY] as any));

            document.getElementById("d3test").innerHTML = "";
            document.getElementById("d3test").appendChild(chart);
            document.getElementById("d3test").appendChild(focus);
        }


        function createRects(svg: d3.Selection<SVGSVGElement, undefined, null, undefined>, x: Function, mini: boolean, height: number) {
            svg.selectAll('.rects').remove();

            const betweenPadding = 2;
            const bottom = height - betweenPadding; // Padding

            const colors = [
                '#da9090',
                '#E7A8E3',
                '#90DAD9',
                '#F2DDC0',
                '#F3BEBC',
                '#A09CF3',
                '#ADEAC3',
                '#c4da90',
            ]

            const graphRecurrings: GraphRecurring[] = recurrings.map((x, i) => {
                return {
                    start: new Date(x.start),
                    end: new Date(x.end),
                    level: -1,
                    name: x.name,
                    color: null,
                }
            });

            const sortedRecurrings = graphRecurrings.sort((a, b) =>
                a.start.getTime() - b.start.getTime());

            // Slow overlap algo
            let ai = 0;
            for (let a of sortedRecurrings) {
                let level = 0;
                a.color = colors[ai % (colors.length)];

                for (let b of sortedRecurrings) {
                    const overlap = a.end > b.start && a.start < b.end;

                    if (overlap && level === b.level)
                        level++;
                }

                a.level = level;
                ai++;

            }

            const rects = svg.append('g')
                .attr('class', 'rects');

            if (!mini) {
                rects.attr("clip-path", "url(#rectClip)");
            }

            for (let recurring of sortedRecurrings) {
                const rectLeft = x(recurring.start);
                const rectRight = x(recurring.end);
                const rectHeight = !mini ? 20 : 5;
                const cornerRadius = !mini ? 5 : 2;

                const rectWidth = rectRight - rectLeft;
                let y = bottom - rectHeight;
                y -= recurring.level * (rectHeight + betweenPadding);

                const g = rects.append('g')
                    .attr('class', 'rect')
                    .attr('transform', `translate(${rectLeft},${y})`)




                g.append('rect')
                    .attr('rx', cornerRadius)
                    .attr('rx', cornerRadius)
                    .attr('width', rectWidth)
                    .attr('height', rectHeight)
                    .attr('fill', recurring.color);

                if (!mini) {
                    g.append('text')
                        .attr('x', 6)
                        .attr('y', 15)
                        .attr('fill', 'black')
                        .attr('font-size', "14px")
                        .text(recurring.name);
                }
            }
        }

        update();
    }

    if (error) {
        return <div className="alert alert-danger" role="alert">
                {error}
            </div>;
    }

    if (!recurrings) {
        return <p>Loading...</p>
    }

    return <div id="d3test">

    </div>
}

type GraphRecurring = {
    start: Date;
    end: Date;
    color: string;
    name: string;

    level?: number;
}
