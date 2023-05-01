<script lang="ts">
  import type { Payment } from "$lib/data_types";
  // import { draw } from "svelte/types/runtime/transition";

  import { draw } from "svelte/transition";

  export let payments: Payment[];

  $: points = getPositions(payments);
  $: maxAmount = 0;

  const width = 1024;
  const height = width / 4;
  const fontSize = height / 20;

  type Point = {
    x: number;
    y: number;
  };

  function getPositions(payments: Payment[]): Point[] {
    let points: Point[] = [];

    for (const payment of payments) {
      points.push({
        x: (payment.timestamp.getDate() * width) / 31,
        y: -payment.amount / 100,
      });
    }

    points.sort((a, b) => a.x - b.x);

    let lastPoint: Point = { x: 0, y: 0 };
    let addedPoints: Point[] = [lastPoint];

    maxAmount = 0;

    for (const point of points) {
      point.y += lastPoint.y;
      if (Math.abs(point.y) > maxAmount) {
        maxAmount = Math.abs(point.y);
      }
      addedPoints.push(
        {
          x: point.x,
          y: lastPoint.y,
        },
        point
      );
      lastPoint = point;
    }

    maxAmount *= 1.1;

    for (let point of addedPoints) {
      point.y = (point.y * height) / maxAmount;
    }

    return addedPoints;
  }

  $: path = getPath(points);

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
      let position = (day * width) / 31;
      points.push({ position, text: day.toString() });
      day += 1;
    }

    return points;
  }

  $: amountPoints = getAmountPoints(maxAmount);

  function getAmountPoints(maxAmount: number): TextPoint[] {
    let points: TextPoint[] = [];

    let step = Math.floor(maxAmount / 50) * 10;

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

<svg viewBox="0 {-height} {width} {height * 2}">
  <line class="helper" x1="0" x2={width} y1="0" y2="0" stroke-width="2" />

  {#each dayPoints as p}
    <circle class="helper" cx={p.position} cy="0" r="1" />
    <text
      class="helper"
      x={p.position - fontSize / 2}
      y={fontSize*1.5}
      font-size={fontSize}
    >
      {p.text}
    </text>
  {/each}

  {#each amountPoints as p}
    <circle class="helper" cy={p.position} cx="0" r="1" />
    <text class="helper" x={fontSize/2} y={p.position + fontSize/4} font-size={fontSize}>
      {p.text}
    </text>
    <line
      class="helper-line"
      x1={fontSize*4}
      x2={width}
      y1={p.position}
      y2={p.position}
      stroke-width="2"
    />
  {/each}

  <path d={path} stroke-width="4" stroke="white" fill="none" />

  {#each points as p}
    <circle cx={p.x} cy={p.y} r="4" fill="white" />
  {/each}
</svg>

<style>
  svg {
    background-color: var(--surface);
    align-self: center;
    border-radius: var(--small);
    width: 50%;
    aspect-ratio: 2;
    padding: var(--small);
  }

  .helper {
    stroke: #ffffffaa;
    fill: #ffffffaa;
  }

  .helper-line {
    stroke: #ffffff60;
  }
</style>
