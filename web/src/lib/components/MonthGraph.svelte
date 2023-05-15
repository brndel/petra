<script lang="ts">
  import type { Payment } from "$lib/data_types";
  import { readable } from "svelte/store";

  export let payments: Payment[];

  type Point = {
    x: number;
    y: number;
  };

  $: paymentLines = getPaymentLines(payments);
  $: maxAmount = 0;

  const width = 1024;
  const height = width / 4;
  const fontSize = height / 20;
  const monthWidth = width / 32;

  function connectPoints(points: Point[]): { line: Point[]; maxValue: number } {
    let line: Point[] = []; 
    let maxValue = 0;

    let lastPoint: Point = { x: 0, y: 0 };

    for (const point of points) {
      let y = lastPoint.y + point.y;
      if (Math.abs(y) > maxValue) {
        maxValue = Math.abs(y);
      }
      line.push(
        {
          x: point.x,
          y: lastPoint.y,
        },
        {
          x: point.x,
          y,
        }
      );
      lastPoint = {
        x: point.x,
        y,
      };
    }

    return { line, maxValue };
  }

  function getPaymentLines(payments: Payment[]): {
    real: Point[];
    calulated: Point[];
  } {
    let days = new Map<number, Payment[]>();

    for (const payment of [...payments].reverse()) {
      let day = payment.timestamp.getDate();
      let dayArray = days.get(day);

      if (dayArray === undefined) {
        dayArray = [];
        days.set(day, dayArray);
      }

      dayArray.push(payment);
    }

    let realPoints: Point[] = [];
    let calculatedPoints: Point[] = [];

    for (const day of [...days.keys()].sort((a, b) => a - b)) {
      let payments = days.get(day)!;

      for (let i = 0; i < payments.length; i++) {
        let payment = payments[i];
        let offset = i / payments.length;
        let x = (day + offset) * monthWidth;

        calculatedPoints.push({
          x,
          y: -(payment.amount + payment.repayAmount) / 100,
        });

        if (payment.isOwner) {
          realPoints.push({
            x,
            y: -payment.amount / 100,
          });
        }
      }
    }

    let { line: realLine, maxValue: maxValue1 } = connectPoints(realPoints);
    let { line: calculatedLine, maxValue: maxValue2 } =
      connectPoints(calculatedPoints);

    maxAmount = Math.max(maxValue1, maxValue2) * 1.1;

    for (let point of realLine) {
      point.y = (point.y * height) / maxAmount;
    }

    for (let point of calculatedLine) {
      point.y = (point.y * height) / maxAmount;
    }

    console.log("finished", realLine, calculatedLine);

    return { real: realLine, calulated: calculatedLine };
  }

  $: calculatedPath = getPath(paymentLines.calulated);
  $: realPath = getPath(paymentLines.real);

  function getPath(points: Point[]): string {
    let path = "M 0 0 ";

    for (const point of points) {
      path += `L ${point.x} ${point.y} `;
    }

    return path;
  }

  type TextPoint = {
    position: number;
    text: string;
  };

  const dayPoints = getDayPoints();

  function getDayPoints(): TextPoint[] {
    let points: TextPoint[] = [];

    let day = 0;
    while (day <= 31) {
      let position = day * monthWidth;
      points.push({ position, text: day.toString() });
      day += 1;
    }

    return points;
  }

  $: amountPoints = getAmountPoints(maxAmount);

  function getAmountPoints(maxAmount: number): TextPoint[] {
    let points: TextPoint[] = [];

    let step = Math.max(1, Math.floor(maxAmount / 50) * 10);

    let amount = 0;

    while (amount < maxAmount) {
      amount += step;
      points.push(
        {
          position: (-amount / maxAmount) * height,
          text: amount.toString(),
        },
        {
          position: (amount / maxAmount) * height,
          text: (-amount).toString(),
        }
      );
    }

    return points;
  }
</script>

<div class="col container">
  <svg viewBox="0 {-height} {width} {height * 2}">
    <g>
      <line class="helper" x1="0" x2={width} y1="0" y2="0" stroke-width="2" />
      {#each dayPoints as p}
        <circle class="helper" cx={p.position} cy="0" r="1" />
        <text
          class="helper"
          x={p.position - fontSize / 2}
          y={fontSize * 1.5}
          font-size={fontSize}
        >
          {p.text}
        </text>
      {/each}
    </g>

    <g>
      {#each amountPoints as p}
        <circle class="helper" cy={p.position} cx="0" r="1" />
        <text
          class="helper"
          x={fontSize / 2}
          y={p.position + fontSize / 4}
          font-size={fontSize}
        >
          {p.text}
        </text>
        <line
          class="helper-line"
          x1={fontSize * 4}
          x2={width}
          y1={p.position}
          y2={p.position}
          stroke-width="2"
        />
      {/each}
    </g>

    <path d="{realPath} V 0" class="amount-fill real" />
    <path d={realPath} class="amount-line real" />

    <path d="{calculatedPath} V 0" class="amount-fill" />
    <path d={calculatedPath} class="amount-line" />
  </svg>
  <div class="row labels">
    <div class="label row center">
      <span class="circle real" />
      <span>Echte Zahlungen</span>
    </div>

    <div class="label row center">
      <span class="circle calculated" />
      <span>Berechnete Zahlungen</span>
    </div>
  </div>
</div>

<style lang="scss">
  .container {
    background-color: var(--surface);
    align-self: center;
    border-radius: var(--small);
    width: 50%;
    aspect-ratio: 2;
    padding: var(--small);

    --real: #0051FF;
    --real-trans: #0051FF40;
    --calculated: #EEEEEE;
    --calculated-trans: #EEEEEE40;
  }

  .amount-fill {
    fill: var(--calculated-trans);
    &.real {
      fill: var(--real-trans);
    }
  }

  .amount-line {
    stroke-width: 4;
    stroke: var(--calculated);
    fill: none;
    &.real {
      stroke: var(--real);
    }
  }

  .helper {
    stroke: #ffffffaa;
    fill: #ffffffaa;
  }

  .helper-line {
    stroke: #ffffff60;
  }

  .label {
    background-color: var(--surface-light);
    padding: var(--small);
    border-radius: var(--small);

    & .circle {
      display: flex;
      width: var(--small);
      height: var(--small);
      border-radius: 50%;

      &.real {
        background-color: var(--real);
      }

      &.calculated {
        background-color: var(--calculated);
      }
    }
  }

</style>
