
'use client';
import { useSearchParams } from 'next/navigation';
import { useEffect, useState } from 'react';
import { Table, Typography, Skeleton } from "antd";
const { Title } = Typography;

import { ReactElement } from 'react';
export default function App() {
  const API_BASE = process.env.NEXT_PUBLIC_API_BASE;
  const [dataaa,setDataaa] = useState(null);
  const n = useSearchParams().get("name");
  const [loading, setLoading] = useState<boolean>(true);

  async function rd() {
    if(!n) {
      console.log("111");
    }
    console.log(n);
    const res = await fetch(`${API_BASE}/servers/${n}`)
    //console.log(res.json())
    const j = await res.json();
    const d = j.data;
    const l = j.label;
    const columns = [
      {
        title: "Time",
        dataIndex: "timestamp",
        key: "timestamp"
      },
      {
        title: "Latency",
        dataIndex: "latency",
        key: "latency"
      },
      {
        title: "Player",
        dataIndex: "player",
        key: "player"
      }
    ]
    function r() {
      return (
        <>
          <Title>
            当前服务器: {l}
          </Title>
          <Table dataSource={d} columns={columns} rowKey={(record) => record.timestamp}/>
        </>

    );
    };
    setDataaa(r());
    setLoading(false);
  }
  function renderSkeletons() {
    return (
      <Skeleton/>
    )
  }
  useEffect(() => {
    rd();
  },[])
  return (
  <div>
      {loading ? renderSkeletons() : dataaa}
  </div>
  );
}
