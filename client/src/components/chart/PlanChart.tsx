import styles from "./PlanChart.module.scss"
import classNames from "classnames";
import * as d3 from "d3";
import {useEffect, useState} from "react";
import React from "react";

const cx = classNames.bind(styles);

type PlanChartProps = {

};

export function PlanChart(props: PlanChartProps) {
    const focusHeight = 100;
    const height = 440;
    const width = 1000;
    const margin = ({top: 20, right: 20, bottom: 30, left: 40});
    const [data, setData] = useState<any>();

    useEffect(() => {
        getData();
    }, []);

    if (!data) {
        return <p>Loading...</p>
    }

    async function getData() {
        const d = await d3.csv("/data.csv");
        console.log(d);
        const tmpData = Object.assign((d).map(({date, close}) => ({date, value: close})), {y: "↑ Close $"});
        setData(tmpData);
    }

    function createChart() {
        const svg = d3.create("svg")
            .attr("viewBox", [0, 0, width, height] as any)
            .style("display", "block");

        const clipId = "clip" + Math.random().toString();

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
            .attr("clip-path", clipId)
            .attr("fill", "steelblue");

        return Object.assign(svg.node(), {
            update(focusX, focusY) {
                gx.call(xAxis, focusX, height);
                gy.call(yAxis, focusY, data.y);
                path.attr("d", area(focusX, focusY) as any);
            }
        });
    }

    const chart = createChart();



    function update() {
        const [minX, maxX] = focus as any;
        const maxY = d3.max(data, (d: any) => minX <= d.date && d.date <= maxX ? d.value as any : NaN);
        chart.update(x.copy().domain(focus as any), y.copy().domain([0, maxY] as any));
    }


    function createFocus() {
        const svg = d3.create("svg")
            .attr("viewBox", [0, 0, width, height] as any)
            .style("display", "block");

        const clipId = "clip" + Math.random().toString();

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
            .attr("clip-path", clipId)
            .attr("fill", "steelblue");

        return Object.assign(svg.node(), {
            update(focusX, focusY) {
                gx.call(xAxis, focusX, height);
                gy.call(yAxis, focusY, data.y);
                path.attr("d", area(focusX, focusY));
            }
        });
    }

    const focus = createFocus();
    const area = (x, y) => d3.area()
        .defined((d: any) => !isNaN(d.value))
        .x((d: any)  => x(d.date))
        .y0(y(0))
        .y1((d: any)  => y(d.value) );

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


    setTimeout(() => {
       document.getElementById("d3test").appendChild(chart);
    });

    return <div id="d3test">

    </div>
}
