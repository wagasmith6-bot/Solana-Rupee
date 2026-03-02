import { useEffect, useState } from "react";
import { fetchTop20 } from "./api";

export default function Dashboard() {
  const [pools, setPools] = useState([]);
  useEffect(() => {
    fetchTop20().then(data => setPools(data.data));
  }, []);

  return (
    <div>
      <h1>Mainnet DAMM v2 Top 20 Pools</h1>
      {pools.map(pool => (
        <div key={pool.address}>
          <p>{pool.baseMint} / {pool.quoteMint}</p>
          <p>Liquidity: ${pool.liquidity.toLocaleString()}</p>
        </div>
      ))}
    </div>
  );
}