import styles from "./PlanChart.module.scss"
import classNames from "classnames";
import * as d3 from "d3";
import {curveBasis} from "d3";
import React, {useEffect, useRef, useState} from "react";
import {PlanApi, Recurring, RecurringApi, Event, RecurringNewPayload, TimeseriesApi, Plan, Allocation} from "../../api";
import handleFetchError from "../../hooks/handleFetchError";
import {addDays, formatPrice} from "../../Helpers";
import {RecurringDialog} from "../recurring/RecurringDialog";
import {RecurringType} from "../recurring/RecurringType";
import {AllocationEditorDialog} from "../allocation/AllocationEditorDialog";
import {epochToDate} from "../recurring/RecurringHelpers";
import {SimulateEventDialog} from "../events/SimulateEventDialog";

const cx = classNames.bind(styles);

type PlanChartProps = {

};

const recurringApi = new RecurringApi();
const planApi = new PlanApi();

export function PlanChart(props: PlanChartProps) {
    const focusHeight = 100;
    const height = 440;
    const width = 1000;
    const margin = ({top: 0, right: 20, bottom: 30, left: 40});
    const [loading, setLoading] = useState<boolean>(true);
    const [accountRecurrings, setAccountRecurrings] = useState<Recurring[]>([]);
    const [error, setError] = useState<string>();
    const scaleRefX = useRef<any>();
    const scaleRefY = useRef<any>();
    const dataRef = useRef<any>();
    const [menuOpen, setMenuOpen] = useState<{x: number, y: number} | null>(null);
    const [menuDate, setMenuDate] = useState<Date>();
    const [mouseX, setMouseX] = useState<number>(null);
    const createRectsRef = useRef<any>();
    const svgRef = useRef<any>();
    const focusSvgRef = useRef<any>();
    const updateRef = useRef<any>();
    const [totalValue, setTotalValue] = useState<number>(null);
    const [mouseValue, setMouseValue] = useState<number>(null);
    const [currentDate, setCurrentDate] = useState<Date>(null);
    const [recurringDialogOpen, setRecurringDialogOpen] = useState<boolean>(false);
    const [recurringDialogEditing, setRecurringDialogEditing] = useState<Recurring>(null);
    const [recurringDialogMode, setRecurringDialogMode] = useState<RecurringType>();
    const [allocationDialogOpen, setAllocationDialogOpen] = useState<boolean>(false);
    const [allocationDialogEditing, setAllocationDialogEditing] = useState<Allocation>(null);
    const [eventsDialogEditing, setEventsDialogEditing] = useState<Event>(null);
    const [eventDialogOpen, setEventDialogOpen] = useState<boolean>(false);
    const [plan, setPlan] = useState<Plan>();

    useEffect(() => {
        fetchEverything();
    }, []);

    async function fetchEverything() {
        setLoading(true);
        await createCharts();
        await getRecurrings();
        setLoading(false);
    }

    useEffect(() => {
        if (updateRef.current)
            updateRef.current();
    }, [accountRecurrings])

    useEffect(() => {
        const handler = () => {
            setMenuOpen(null);
        };
        document.addEventListener("click", handler);
        return () => {
            document.removeEventListener("click", handler);
        }
    }, []);

    async function getRecurrings() {
        try {
            const recurrings = await recurringApi.getRecurrings();
            setAccountRecurrings(recurrings);
        } catch (e) {
            setError(await handleFetchError(e));
        }
    }

    async function getData() {
        const planData = await planApi.getPlanWithDays({
            days: 365 * 10,
        });
        setPlan(planData.plan);

        const ts = planData.timeseries;
        console.log(planData.plan);

        const predictionStart = epochToDate(ts.start);
        const series = ts.series;
        const data = Object.assign(series.map(({date, net_worth}) =>
            ({date: epochToDate(date), value: net_worth.amount})));
        dataRef.current = data;

        const knownData = data.filter(f => f.date <= predictionStart);
        const predictedData = data.filter(f => f.date >= predictionStart);

        return {data, knownData, predictedData}
    }

    async function updateTimeseries() {
        const {data, knownData, predictedData} = await getData();

        svgRef.current.select("#knownPath")
            .datum(knownData);

        svgRef.current.select("#knownArea")
            .datum(knownData);

        svgRef.current.select("#predictedPath")
            .datum(predictedData);

        focusSvgRef.current.select("#focusKnownData")
            .datum(knownData);

        focusSvgRef.current.select("#focusPredictedData")
            .datum(predictedData);

        updateRef.current();
    }

    async function createCharts() {
        const {data, knownData, predictedData} = await getData();

        setTotalValue(knownData[knownData.length - 1].value);
        setMouseValue(null);

        const area = (x, y) => d3.area()
            .curve(curveBasis)
            .defined((d: any) => !isNaN(d.value))
            .x((d: any)  => x(d.date))
            .y0(y(0))
            .y1((d: any)  => y(Math.max(0, d.value))); // Math.max prevents line from going below the chart

        const line = (x, y) => d3.area()
            .curve(curveBasis)
            .defined((d: any) => !isNaN(d.value))
            .x((d: any)  => x(d.date))
            .y((d: any)  => y(Math.max(0, d.value)));

        const x = d3.scaleUtc()
            .domain(d3.extent(data,  (d: any) => d.date) as any)
            .range([margin.left, width - margin.right]);

        const maxY = d3.max(data,  (d: any) => d.value);
        const y = d3.scaleLinear()
            .domain([0, maxY] as any)
            .range([height - margin.bottom, margin.top]);

        scaleRefY.current = y;

        const xAxis = (g, x, height) => g
            .attr("transform", `translate(0,${height - margin.bottom})`)
            .call(d3.axisBottom(x).ticks(width / 80).tickSizeOuter(0));

        const yAxis = (g, y, title) => g
            .attr("transform", `translate(${margin.left},0)`)
            .call(d3.axisLeft(y).tickFormat(d3.format(".2s")))
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
                .style("display", "block")
                .on("contextmenu", onContextMenu);

            svg.append("linearGradient")
                .attr("id", "areaGradient")
                .attr("x1", 0).attr("y1", "0%")
                .attr("x2", 0).attr("y2", "100%")
                .selectAll("stop")
                .data([
                    {offset: "10%", color: "#21c19c", opacity: 0},
                    {offset: "100%", color: "#21c19c", opacity: 0.4},
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
                .attr("id", "knownPath")
                .attr("clip-path", "url(#" + clipId + ")")
                .attr("class", styles.path + " " + styles["path--known"]);

            const knownArea = svg.append("path")
                .datum(knownData)
                .attr("id", "knownArea")
                .attr("clip-path", "url(#" + clipId + ")")
                .attr("class", styles.area);

            const predictedPath = svg.append("path")
                .datum(predictedData)
                .attr("id", "predictedPath")
                .attr("clip-path", "url(#" + clipId + ")")
                .attr("class", styles.path + " " + styles["path--predicted"]);

            const node = svg.node();

            const mouseLine = svg.append("line")
                .attr("y1", 0)
                .attr("y2", 410)
                .attr("class", styles["mouse-line"]);

            (node as any).update = function(focusX, focusY) {
                scaleRefX.current = focusX;
                scaleRefY.current = focusY;
                gx.call(xAxis, focusX, height);
                gy.call(yAxis, focusY, data.y);
                knownArea.attr("d", area(focusX, focusY) as any);
                knownPath.attr("d", line(focusX, focusY) as any);
                predictedPath.attr("d", line(focusX, focusY) as any);

                createRectsRef.current( svg, focusX, false, height - margin.bottom);
            };

            svgRef.current = svg;
            const mouseArea = svg.append("rect")
                .attr("x", margin.left)
                .attr("y", margin.top)
                .attr("height", 400)
                .attr('pointer-events', 'all')
                .attr("fill", "none")
                .attr("width", 1000).node();
            mouseArea.addEventListener("mousemove",mouseOver);
            mouseArea.addEventListener("mouseleave",mouseLeave);
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

            const defaultSelection = [x(addDays(new Date(), 0)), x(addDays(new Date(), 365))];

            svg.append("g")
                .call(xAxis, x, focusHeight);

            const knownPath =  svg.append("path")
                .datum(knownData)
                .attr("id", "focusKnownData")
                .attr("d", line(x, y.copy().range([focusHeight - margin.bottom, 4])) as any)
                .attr("class", styles.path+ " " + styles["path--known"]);

           const predictedPath  = svg.append("path")
                .datum(predictedData)
                .attr("id", "focusPredictedData")
                .attr("d", line(x, y.copy().range([focusHeight - margin.bottom, 4])) as any)
                .attr("class", styles.path + " " + styles["path--predicted"]);

            const gb = svg.append("g")
                .call(brush)
                .call(brush.move, defaultSelection);


            function brushed({selection}) {
                if (selection) {
                    svg.property("value", selection.map(x.invert, x).map(d3.utcDay.round));
                    svg.dispatch("input");
                    if (updateRef.current) {
                        requestAnimationFrame(() => {
                            update();
                        });
                    }
                }
            }

            function brushended({selection}) {
                if (!selection) {
                    gb.call(brush.move, defaultSelection);
                }
            }
            const node = svg.node();
            (node as any).update = function() {

                knownPath.attr("d", line(x, y.copy().range([focusHeight - margin.bottom, 4])) as any);
                predictedPath.attr("d", line(x, y.copy().range([focusHeight - margin.bottom, 4])) as any);
                createRectsRef.current(focusSvg, x, true, focusHeight - 30);
            };

            focusSvgRef.current = svg;
            return [node, svg];
        }

        const [focus, focusSvg] = createFocus();

        function update() {
            const [minX, maxX] = (focus as any).value as any;
            const maxY = d3.max(dataRef.current, (d: any) => minX <= d.date && d.date <= maxX ? d.value as any : NaN) as any as number;
            (chart as any).update(x.copy().domain((focus as any).value as any),y.copy().domain([0, maxY * 1.5] as any));
            (focus as any).update();
            document.getElementById("d3test").innerHTML = "";
            document.getElementById("d3test").appendChild(chart);
            document.getElementById("d3test").appendChild(focus as any);
        }

        updateRef.current = update;
        update();
    }

    createRectsRef.current = function(svg: d3.Selection<SVGSVGElement, undefined, null,
        undefined>, x: Function, mini: boolean, height: number) {
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
        ];


        const graphRecurrings: GraphRecurring[] = accountRecurrings.map((x, i) => {
            return {
                obj: x,
                start: epochToDate(x.start),
                end: epochToDate(x.end),
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

        // Add the allocation change rectangles
        for (let allocation of plan.allocations) {
            const rectLeft = x(epochToDate(allocation.date));
            const rectRight = x(addDays(epochToDate(allocation.date), 1));
            const rectWidth = Math.max(mini ? 1 : 5 , Math.round(rectRight - rectLeft));
            let y = margin.top;

            const g = rects.append('g')
                .attr('transform', `translate(${rectLeft},${y})`)

            g.append('rect')
                .on('click', () => clickAllocationRect(allocation))
                .attr('class', styles.rect + " " + styles["rect--allocation"])
                .attr('width', rectWidth)
                .attr('height', height);

            if (!mini && rectWidth > 14) {
                g.append('text')
                    .attr('class', styles.rect__text + " " + styles["rect__text--allocation"])
                    .attr('x', (rectWidth / 2) + 1)
                    .attr('y', 150)
                    .attr('fill', 'black')
                    .attr('font-size', '12px')
                    .text("Allocation Change");
            }
        }

        // Add the events
        for (let event of plan.events) {
            const rectLeft = x(epochToDate(event.start));
            const rectRight = x(addDays(epochToDate(event.start), 1));
            const rectWidth = Math.max(mini ? 1 : 5 , Math.round(rectRight - rectLeft));
            let y = margin.top;

            const g = rects.append('g')
                .attr('transform', `translate(${rectLeft},${y})`)

            g.append('rect')
                .on('click', () => clickEventRect(event))
                .attr('class', styles.rect + " " + styles["rect--event"])
                .attr('width', rectWidth)
                .attr('height', height);

            if (!mini && rectWidth > 14) {
                g.append('text')
                    .attr('class', styles.rect__text + " " + styles["rect__text--event"])
                    .attr('x', (rectWidth / 2) + 1)
                    .attr('y', 150)
                    .attr('fill', 'black')
                    .attr('font-size', '12px')
                    .text("Allocation Change");
            }
        }


        // Add the recurring rectangles
        for (let recurring of sortedRecurrings) {
            const rectLeft = x(recurring.start);
            const rectRight = x(recurring.end);
            const rectHeight = !mini ? 20 : 5;
            const cornerRadius = !mini ? 5 : 2;

            const rectWidth = Math.round(rectRight - rectLeft);
            let y = bottom - rectHeight;
            y -= recurring.level * (rectHeight + betweenPadding);

            const g = rects.append('g')
                .attr('transform', `translate(${rectLeft},${y})`)

            g.append('rect')
                .on('click', () => clickRecurringRect(recurring.obj))
                .attr('class', styles.rect)
                .attr('rx', cornerRadius)
                .attr('rx', cornerRadius)
                .attr('width', rectWidth)
                .attr('height', rectHeight)
                .attr('fill', recurring.color);

            if (!mini) {
                const left = rectLeft - margin.left - 6;
                console.log(left, rectWidth);
                const x = (left < 0  && left > -rectWidth + 20 ? -rectLeft + margin.left : 0) + 6;
                const fontSize = Math.min(14, Math.max(11, (rectWidth / 200) * 14));
                g.append('text')
                    .attr('class', styles.rect__text)
                    .attr('x', x)
                    .attr('y', 15 - (14 - fontSize) * 0.5)
                    .attr('fill', 'black')
                    .attr('font-size', fontSize)
                    .text(recurring.name);
            }
        }
    }

    function onContextMenu(e: MouseEvent) {
        if (!scaleRefX.current) return;
        const date = scaleRefX.current.invert(e.offsetX);

        setMenuDate(date);
        setMenuOpen({x: e.offsetX + 5, y: e.offsetY + 10});

        e.preventDefault();
    }

    function mouseOver(e: MouseEvent) {
        if (!scaleRefX.current) return;
        // For some reason the offsetX increases slightly faster than the actual mouse position...
        // this hack fixes it.
        const x = e.offsetX - e.offsetX * 0.06;
        if (x <= 0) return;


        setMouseX(x);
        const date = scaleRefX.current.invert(x);
        const bisect = d3.bisector(function(d) { return (d as any).date; }).left;
        const point = dataRef.current[bisect(dataRef.current as any, date)];

        if (point) {
            setMouseValue(point.value);
            setCurrentDate(point.date);
            svgRef.current.select("." + styles["mouse-line"])
                .attr("opacity", 1)
                .attr("x1", x)
                .attr("x2", x);
        } else {
            mouseLeave();
        }
    }

    function mouseLeave() {
        setMouseX(null);
        setCurrentDate(null);
        setMouseValue(null);
        svgRef.current.select("." + styles["mouse-line"]).attr("x1", -100).attr("x2", -100);
    }

    if (error) {
        return <div className="alert alert-danger" role="alert">
                {error}
            </div>;
    }

    async function recurringDialogClosed(recurring: RecurringNewPayload) {


        if (recurring) {
            // Check if the recurring is attched to the account or plan.
            var isAccountRecurring = false;
            if (recurringDialogEditing)
                isAccountRecurring = !!accountRecurrings.find(r => r._id.$oid === recurringDialogEditing._id.$oid);

            console.log("IS account recurring?", isAccountRecurring);

            if (isAccountRecurring) {

                if (recurringDialogEditing) {
                    setLoading(true);
                    await recurringApi.updateRecurring({
                        recurringNewPayload: recurring,
                        id: recurringDialogEditing._id.$oid,
                    });
                    setLoading(false);
                    Object.assign(recurringDialogEditing, recurring);
                    setAccountRecurrings([...accountRecurrings]);
                } else {
                    const result = await recurringApi.newRecurring({
                        recurringNewPayload: recurring
                    });
                    setAccountRecurrings([...accountRecurrings, result]);
                }
            } else {
                // Edit the relevant plan recurring OR append to the plan recurrings.
                if (recurringDialogEditing) {
                    var planRecurring = plan.recurrings.find(r => r._id.$oid === recurringDialogEditing._id.$oid);
                    var index = plan.recurrings.indexOf(planRecurring);
                    // Replace the existing one with the new one.
                    plan.recurrings.splice(index, 1, recurringDialogEditing);
                    await planApi.newPlan({
                        planNewPayload: {
                            ...plan,
                            recurrings: [...plan.recurrings],
                        }
                    });
                } else {
                    await planApi.newPlan({
                        planNewPayload: {
                            ...plan,
                            recurrings: [...plan.recurrings, recurring],
                        }
                    });
                }
            }

            updateRef.current();
            setLoading(true);
            await updateTimeseries();
            setLoading(false);
        }
        setRecurringDialogEditing(null);
        setRecurringDialogOpen(false);
    }

    async function deleteRecurring(recurring: Recurring) {
        await recurringApi.deleteRecurring({
            id: recurring._id.$oid
        });
        setAccountRecurrings([...accountRecurrings.filter(r => r._id.$oid !== recurring._id.$oid)]);
    }

    async function eventDialogClosed(events: Event[]) {
        setEventDialogOpen(false);
        setEventsDialogEditing(null);

        // The editor was closed without modifying anything, e.g the X was clicked.
        if (!events) return;

        await planApi.newPlan({
            planNewPayload: {
                ...plan,
                events,
            }
        });

        setLoading(true);
        await updateTimeseries();
        setLoading(false);
    }

    function menuAddExpense() {
        setRecurringDialogOpen(true);
        setRecurringDialogMode(RecurringType.Expense);
    }

    function menuAddIncome() {
        setRecurringDialogOpen(true);
        setRecurringDialogMode(RecurringType.Income);
    }

    function clickRecurringRect(recurring: Recurring) {
        setRecurringDialogEditing(recurring);
        setRecurringDialogMode(recurring.amount < 0 ? RecurringType.Expense : RecurringType.Income);

        setRecurringDialogOpen(true);
    }

    function clickAllocationRect(allocation: Allocation) {
        setAllocationDialogEditing(allocation);
        setAllocationDialogOpen(true);
    }

    function clickEventRect(event: Event) {
        setEventsDialogEditing(event);
        setEventDialogOpen(true);
    }

    function menuModifyAllocations() {
        setAllocationDialogOpen(true);
    }

    function menuSimulateEvent() {
        setEventDialogOpen(true);
    }

    function getDateString() {
        if (!currentDate) return 'Today';
        var options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };
        return currentDate.toLocaleDateString("en-US", options);
    }

    function renderPercentDifference() {
        if (!mouseValue) return null;
        const percentDifference = ((mouseValue - totalValue) / totalValue) * 100;
        if (percentDifference >= 0) {
            return <span className="text-success">+{percentDifference.toFixed(2)}%</span>
        } else {
            return <span className="text-danger">{percentDifference.toFixed(2)}%</span>
        }
    }

    async function allocationEditorClosed(allocations: Allocation[]) {
       setAllocationDialogOpen(false);
       setAllocationDialogEditing(null);

       // The editor was closed without modifying anything, e.g the X was clicked.
       if (!allocations) return;

       await planApi.newPlan({
           planNewPayload: {
               ...plan,
               allocations,
           }
       });

       setLoading(true);
       await updateTimeseries();
       setLoading(false);
    }

    return <>
        {
            <div className={cx(styles.loader, {
                [styles["loader--active"]]: loading
            })}>
              <i className="fa fa-spinner fa-spin fa-fw"/>
              <span className="ml-3">Loading...</span>
            </div>
        }
        {
            plan &&
              <>
            <AllocationEditorDialog allocations={plan.allocations}
                                    editing={allocationDialogEditing} creating={menuDate} show={allocationDialogOpen}
                                    onClose={allocations => allocationEditorClosed(allocations)}/>
            <RecurringDialog startDate={menuDate} show={recurringDialogOpen} mode={recurringDialogMode} onClose={r => recurringDialogClosed(r)}
                             onDelete={r => deleteRecurring(r)}
            editing={recurringDialogEditing}/>
            <SimulateEventDialog events={plan.events} show={eventDialogOpen} editing={eventsDialogEditing} creating={menuDate} onClose={e => eventDialogClosed(e)}/>
              </>
        }
        {totalValue !== null && <div className={styles.current}>
            <h2 className={styles.value}>{formatPrice(mouseValue ?? totalValue)} <span className={styles.difference}>{renderPercentDifference()}</span></h2>
            <h5 className="text-secondary">{getDateString()}</h5>
            </div>
        }

        <div id="d3test">
        </div>
        {
            menuOpen && <div className={styles.menu}
            style={{left: menuOpen.x + "px", top: menuOpen.y + "px"}}>
              <div>
                <strong>{menuDate.toDateString()}</strong>
              </div>
              <ul>
                <li onClick={menuAddExpense}>Add Expense</li>
                <li onClick={menuAddIncome}>Add Income</li>
                <li onClick={menuModifyAllocations}>Modify Allocations</li>
                <li onClick={menuSimulateEvent}>Simulate Event</li>
              </ul>
            </div>
        }
    </>
}

type GraphRecurring = {
    obj: Recurring;
    start: Date;
    end: Date;
    color: string;
    name: string;

    level?: number;
}
