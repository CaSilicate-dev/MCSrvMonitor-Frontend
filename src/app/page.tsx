
'use client';
import '@ant-design/v5-patch-for-react-19';

import { useSearchParams } from 'next/navigation';
import { useEffect, useState } from 'react';
import { Card, Space, Row, Tooltip, Typography, Skeleton, notification, message } from "antd";
const { Title, Text } = Typography;

import { ReactElement } from 'react';
import Link from "next/link";
import { Color } from 'antd/es/color-picker';
export default function App() {
  const API_BASE = process.env.NEXT_PUBLIC_API_BASE;
  const [dataaa,setDataaa] = useState(null);
  const n = useSearchParams().get("name");
  const [cards,setCards] = useState<ReactElement[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  function openNotification(type, msg) {
    notification[type]({
      message: msg,
    });
  };

  async function f() {
    try{
      if(!n) {
        
      }
      console.log(n);
      const res = await fetch(`${API_BASE}/list`)
      const j = await res.json();
      const d = j.namelist;

      const result: ReactElement[] = [];
      for (const [index, item] of d.entries()) {
        console.log(item);
        const cres = await fetch(`${API_BASE}/serverod/${item}`)
        const cj = await cres.json();
        console.log(cj)
        const clatency = cj.data[0].latency;
        const cplayer = cj.data[0].player;
        const cpl = cj.data[0].playerlist
        var ccl = "#fff";
        if (clatency > 0 && clatency < 150) {
          ccl = "#90ee90"
        } else if (clatency >= 150) {
          ccl = "#ff00ff"
        }
        else {
          ccl = "#ff0000"
        }

        result.push(
          <Link href={`/server/?name=${item}`}>
            <Card key={index} /*extra={<Link href={`/server/?name=${item}`}>More</Link>}*/ title={`Server: ${item}`} style={{
              width: 300,
              boxShadow: "0 2px 8px rgba(0, 0, 0, 0.1)"
            }}>
              <p>
                当前延迟: <Text strong style={{color: ccl}}>{clatency}</Text>
              </p>
              <Tooltip title={cpl}>
                <p>当前玩家数量: <Text strong>{cplayer}</Text></p>
              </Tooltip>
            </Card>
          </Link>

        )
        setLoading(false);
      }
      setCards(result);
    } catch(error) {
      openNotification('error',error.message);
    }
  }
  function renderSkeletons() {
    return (
      <Skeleton/>
    )
  }
  useEffect(() => {
    f();
    const timer = setInterval(f, 3000);
    return () => clearInterval(timer);
  },[])
  /*return (
  <div>
    <Row gutter={[16, 16]}>
      <Space>
        {loading ? renderSkeletons() : cards}
        <Skeleton active />
      </Space>
    </Row>
  </div>
  );*/
  return (
  <div>
    <Title>Minecraft 服务器监控器</Title>
    {loading ? renderSkeletons() : (
      <Row gutter={[16, 16]}>
        <Space>
          {loading ? renderSkeletons() : cards}
        </Space>
      </Row>
    )}
  </div>
  );
}
