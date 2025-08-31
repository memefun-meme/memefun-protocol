import React, { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { 
    TrendingUp, 
    DollarSign, 
    Shield, 
    AlertTriangle,
    Plus,
    Minus,
    Activity,
    PieChart
} from 'lucide-react';

const TreasuryYieldDashboard: React.FC = () => {
    const { publicKey } = useWallet();
    const [loading, setLoading] = useState(true);
    const [totalValueLocked, setTotalValueLocked] = useState(0);
    const [totalYieldEarned, setTotalYieldEarned] = useState(0);
    const [currentApy, setCurrentApy] = useState(0);
    const [activePositions, setActivePositions] = useState(0);
    const [riskLevel, setRiskLevel] = useState('Medium');

    useEffect(() => {
        // Simulate loading and data fetching
        setTimeout(() => {
            setLoading(false);
            setTotalValueLocked(2500000); // 2.5M SOL
            setTotalYieldEarned(125000); // 125K SOL
            setCurrentApy(12.5);
            setActivePositions(8);
        }, 1000);
    }, []);

    const formatNumber = (num: number): string => {
        if (num >= 1000000) return (num / 1000000).toFixed(1) + 'M';
        if (num >= 1000) return (num / 1000).toFixed(1) + 'K';
        return num.toString();
    };

    const getRiskColor = (risk: string) => {
        switch (risk) {
            case 'Low': return 'text-green-600';
            case 'Medium': return 'text-yellow-600';
            case 'High': return 'text-orange-600';
            case 'VeryHigh': return 'text-red-600';
            default: return 'text-gray-600';
        }
    };

    if (loading) {
        return (
            <div className="flex items-center justify-center min-h-screen">
                <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-primary"></div>
            </div>
        );
    }

    return (
        <div className="container mx-auto px-4 py-8">
            <div className="mb-8">
                <h1 className="text-4xl font-bold mb-2">💰 Treasury Yield Farming</h1>
                <p className="text-muted-foreground">
                    DeFi integration for maximizing treasury returns and staker rewards
                </p>
            </div>

            {/* Overview Cards */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">Total Value Locked</CardTitle>
                        <DollarSign className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">{formatNumber(totalValueLocked)} SOL</div>
                        <p className="text-xs text-muted-foreground">
                            +2.5% from last week
                        </p>
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">Total Yield Earned</CardTitle>
                        <TrendingUp className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">{formatNumber(totalYieldEarned)} SOL</div>
                        <p className="text-xs text-muted-foreground">
                            +15.3% from last month
                        </p>
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">Current APY</CardTitle>
                        <Activity className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">{currentApy.toFixed(1)}%</div>
                        <p className="text-xs text-muted-foreground">
                            +0.8% from last week
                        </p>
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                        <CardTitle className="text-sm font-medium">Active Positions</CardTitle>
                        <PieChart className="h-4 w-4 text-muted-foreground" />
                    </CardHeader>
                    <CardContent>
                        <div className="text-2xl font-bold">{activePositions}</div>
                        <p className="text-xs text-muted-foreground">
                            Across 4 protocols
                        </p>
                    </CardContent>
                </Card>
            </div>

            {/* Risk Management */}
            <Card className="mb-8">
                <CardHeader>
                    <CardTitle className="flex items-center gap-2">
                        <Shield className="w-5 h-5" />
                        Risk Management
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div>
                            <div className="flex items-center justify-between mb-2">
                                <span className="text-sm font-medium">Total Risk Level</span>
                                <Badge className={getRiskColor(riskLevel)}>{riskLevel}</Badge>
                            </div>
                            <Progress value={45} className="h-2" />
                            <p className="text-xs text-muted-foreground mt-1">
                                45% of maximum risk (50%)
                            </p>
                        </div>
                        
                        <div>
                            <div className="flex items-center justify-between mb-2">
                                <span className="text-sm font-medium">Protocol Risk</span>
                                <span className="text-sm">18%</span>
                            </div>
                            <Progress value={18} className="h-2" />
                            <p className="text-xs text-muted-foreground mt-1">
                                Max: 20% per protocol
                            </p>
                        </div>
                        
                        <div>
                            <div className="flex items-center justify-between mb-2">
                                <span className="text-sm font-medium">Token Allocation</span>
                                <span className="text-sm">25%</span>
                            </div>
                            <Progress value={25} className="h-2" />
                            <p className="text-xs text-muted-foreground mt-1">
                                Max: 30% per token
                            </p>
                        </div>
                    </div>
                </CardContent>
            </Card>

            {/* Active Positions */}
            <Card className="mb-8">
                <CardHeader>
                    <CardTitle>Active Yield Positions</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="space-y-4">
                        {[
                            { protocol: 'Raydium', token: 'SOL', amount: 500000, apy: 15.2, risk: 'Low', earned: 25000 },
                            { protocol: 'Orca', token: 'USDC', amount: 300000, apy: 8.5, risk: 'Low', earned: 8500 },
                            { protocol: 'Jupiter', token: 'SOL', amount: 400000, apy: 12.8, risk: 'Medium', earned: 16000 },
                            { protocol: 'Marinade', token: 'mSOL', amount: 600000, apy: 6.2, risk: 'Low', earned: 12000 },
                        ].map((position, index) => (
                            <div key={index} className="flex items-center justify-between p-4 border rounded-lg">
                                <div className="flex items-center gap-4">
                                    <div className="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center">
                                        <DollarSign className="w-6 h-6 text-primary" />
                                    </div>
                                    <div>
                                        <h4 className="font-semibold">{position.protocol}</h4>
                                        <p className="text-sm text-muted-foreground">
                                            {position.token} • {formatNumber(position.amount)} deposited
                                        </p>
                                    </div>
                                </div>
                                
                                <div className="flex items-center gap-6">
                                    <div className="text-right">
                                        <p className="font-semibold text-green-600">{position.apy}% APY</p>
                                        <p className="text-sm text-muted-foreground">
                                            Earned: {formatNumber(position.earned)}
                                        </p>
                                    </div>
                                    
                                    <Badge className={getRiskColor(position.risk)}>
                                        {position.risk}
                                    </Badge>
                                    
                                    <div className="flex gap-2">
                                        <Button size="sm" variant="outline">
                                            <Plus className="w-4 h-4" />
                                        </Button>
                                        <Button size="sm" variant="outline">
                                            <Minus className="w-4 h-4" />
                                        </Button>
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                </CardContent>
            </Card>

            {/* Yield Distribution */}
            <Card className="mb-8">
                <CardHeader>
                    <CardTitle>Yield Distribution</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div className="text-center p-4 bg-green-50 rounded-lg">
                            <div className="text-2xl font-bold text-green-600">70%</div>
                            <p className="text-sm font-medium">To Stakers</p>
                            <p className="text-xs text-muted-foreground">
                                {formatNumber(totalYieldEarned * 0.7)} SOL distributed
                            </p>
                        </div>
                        
                        <div className="text-center p-4 bg-blue-50 rounded-lg">
                            <div className="text-2xl font-bold text-blue-600">20%</div>
                            <p className="text-sm font-medium">To Treasury</p>
                            <p className="text-xs text-muted-foreground">
                                {formatNumber(totalYieldEarned * 0.2)} SOL retained
                            </p>
                        </div>
                        
                        <div className="text-center p-4 bg-orange-50 rounded-lg">
                            <div className="text-2xl font-bold text-orange-600">10%</div>
                            <p className="text-sm font-medium">Emergency Fund</p>
                            <p className="text-xs text-muted-foreground">
                                {formatNumber(totalYieldEarned * 0.1)} SOL reserved
                            </p>
                        </div>
                    </div>
                </CardContent>
            </Card>

            {/* Performance Metrics */}
            <Card>
                <CardHeader>
                    <CardTitle>Performance Metrics</CardTitle>
                </CardHeader>
                <CardContent>
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                        <div className="text-center">
                            <div className="text-lg font-semibold">Daily Yield</div>
                            <div className="text-2xl font-bold text-green-600">
                                {formatNumber(5000)} SOL
                            </div>
                        </div>
                        
                        <div className="text-center">
                            <div className="text-lg font-semibold">Weekly Yield</div>
                            <div className="text-2xl font-bold text-green-600">
                                {formatNumber(35000)} SOL
                            </div>
                        </div>
                        
                        <div className="text-center">
                            <div className="text-lg font-semibold">Monthly Yield</div>
                            <div className="text-2xl font-bold text-green-600">
                                {formatNumber(150000)} SOL
                            </div>
                        </div>
                        
                        <div className="text-center">
                            <div className="text-lg font-semibold">Sharpe Ratio</div>
                            <div className="text-2xl font-bold text-blue-600">
                                1.85
                            </div>
                        </div>
                    </div>
                </CardContent>
            </Card>

            {/* Action Buttons */}
            <div className="flex gap-4 mt-8">
                <Button className="flex items-center gap-2">
                    <Plus className="w-4 h-4" />
                    Add Position
                </Button>
                <Button variant="outline" className="flex items-center gap-2">
                    <TrendingUp className="w-4 h-4" />
                    Harvest Yield
                </Button>
                <Button variant="outline" className="flex items-center gap-2">
                    <AlertTriangle className="w-4 h-4" />
                    Emergency Withdraw
                </Button>
            </div>
        </div>
    );
};

export default TreasuryYieldDashboard;
