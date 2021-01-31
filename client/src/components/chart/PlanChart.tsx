import styles from "./PlanChart.module.scss"
import classNames from "classnames";
import * as d3 from "d3";
import {useEffect, useState} from "react";
import React from "react";
import { curveBasis } from "d3";

const cx = classNames.bind(styles);

type PlanChartProps = {

};

export function PlanChart(props: PlanChartProps) {
    const focusHeight = 100;
    const height = 440;
    const width = 1000;
    const margin = ({top: 20, right: 20, bottom: 30, left: 40});

    useEffect(() => {
        getData();
    }, []);

    async function getData() {
        const d = await d3.csv("/data.csv");
        const data = Object.assign((d).map(({date, close}) =>
            ({date: new Date(date), value: parseFloat(close)})), {y: "↑ Close $"});

        /*
        const area = (x, y) => d3.area()
            .defined((d: any) => !isNaN(d.value))
            .x((d: any)  => x(d.date))
            .y0(y(0))
            .y1((d: any)  => y(d.value));

         */

        const line = (x, y) => d3.area()
            .curve(curveBasis)
            .defined((d: any) => !isNaN(d.value))
            .x((d: any)  => x(d.date))
            .y((d: any)  => y(d.value));

        const x = d3.scaleUtc()
            .domain(d3.extent(data,  (d: any) => d.date) as any)
            .range([margin.left, width - margin.right]);

        const y = d3.scaleLinear()
            .domain([0, d3.max(data,  (d: any) => d.value)] as any)
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

            const clipId = "clip" + Math.round(Math.random() * 10000);

            svg.append("clipPath")
                .attr("id", clipId)
                .append("rect")
                .attr("x", margin.left)
                .attr("y", 0)
                .attr("height", height)
                .attr("width", width - margin.left - margin.right);

            const gx = svg.append("g");

            const gy = svg.append("g");

            const path = svg.append("path")
                .datum(data)
                .attr("clip-path", "url(#" + clipId + ")")
                .attr("class", styles.path);

            const node = svg.node();
            (node as any).update = function(focusX, focusY) {
                gx.call(xAxis, focusX, height);
                gy.call(yAxis, focusY, data.y);
                path.attr("d", line(focusX, focusY) as any);
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
                .datum(data)
                .attr("d", line(x, y.copy().range([focusHeight - margin.bottom, 4])) as any)
                .attr("class", styles.path);

            const gb = svg.append("g")
                .call(brush)
                .call(brush.move, defaultSelection);

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

        update();
    }

    return <div id="d3test">

    </div>
}
