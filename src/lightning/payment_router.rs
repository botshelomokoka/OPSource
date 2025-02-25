// Lightning Network Payment Router
// Handles route finding and path optimization for payments

use std::sync::{Arc, Mutex};
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;

use crate::lightning::interface::{
    LightningError, LightningResult
};

#[cfg(feature = "ldk")]
use lightning::{
    routing::router::{Route, RouteHop, RouteParameters, Router},
    routing::gossip::NetworkGraph,
    ln::channelmanager::ChannelDetails,
    ln::msgs::RoutingFees
};

/// Payment router for finding paths through the Lightning Network
pub struct PaymentRouter {
    /// Network graph for route finding
    #[cfg(feature = "ldk")]
    network_graph: Option<Arc<NetworkGraph>>,
    
    /// Manual graph for mock implementations
    manual_graph: Mutex<Graph>,
    
    /// Configuration
    config: Arc<crate::config::Config>,
}

/// Simple graph structure for route finding
#[derive(Default)]
struct Graph {
    /// Node edges (pubkey -> [(target_pubkey, channel_id, capacity, fee_base_msat, fee_proportional_millionths)])
    edges: HashMap<String, Vec<(String, String, u64, u32, u32)>>,
    
    /// Channel details (channel_id -> (source, target, capacity, fee_base_msat, fee_proportional_millionths))
    channels: HashMap<String, (String, String, u64, u32, u32)>,
}

/// A route hop in a payment path
#[derive(Clone, Debug, PartialEq)]
pub struct PaymentHop {
    /// Source node
    pub src_node_id: String,
    
    /// Destination node
    pub dest_node_id: String,
    
    /// Channel ID
    pub channel_id: String,
    
    /// Amount to forward (in msats)
    pub amount_msat: u64,
    
    /// Fee to pay (in msats)
    pub fee_msat: u64,
    
    /// CLTV delta to add
    pub cltv_expiry_delta: u32,
}

/// A complete payment route
#[derive(Clone, Debug)]
pub struct PaymentRoute {
    /// The hops in this route
    pub hops: Vec<PaymentHop>,
    
    /// Total amount to send (in msats)
    pub total_amount_msat: u64,
    
    /// Total fee for the route (in msats)
    pub total_fee_msat: u64,
    
    /// Total CLTV expiry delta
    pub total_cltv_expiry_delta: u32,
}

/// Node with distance for pathfinding
#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeWithDistance {
    /// Node public key
    pubkey: String,
    
    /// Fee to reach this node
    fee: u64,
    
    /// Previous node in the path
    prev_node: Option<String>,
    
    /// Channel from previous node
    prev_channel: Option<String>,
}

impl Ord for NodeWithDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.fee.cmp(&self.fee)
    }
}

impl PartialOrd for NodeWithDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PaymentRouter {
    /// Create a new Payment Router
    pub fn new(config: &crate::config::Config) -> Self {
        PaymentRouter {
            #[cfg(feature = "ldk")]
            network_graph: None,
            manual_graph: Mutex::new(Graph::default()),
            config: Arc::new(config.clone()),
        }
    }
    
    /// Find a route from source to destination
    pub fn find_route(
        &self,
        source: &str,
        destination: &str,
        amount_msat: u64,
        max_cltv_expiry: u32,
    ) -> LightningResult<PaymentRoute> {
        #[cfg(feature = "ldk")]
        if let Some(network_graph) = &self.network_graph {
            // In a real implementation, we would use LDK's router to find a path
            // For now, use our manual graph as a fallback
            return self.find_route_manual(source, destination, amount_msat, max_cltv_expiry);
        }
        
        self.find_route_manual(source, destination, amount_msat, max_cltv_expiry)
    }
    
    /// Find a route using our manual graph
    fn find_route_manual(
        &self,
        source: &str,
        destination: &str,
        amount_msat: u64,
        max_cltv_expiry: u32,
    ) -> LightningResult<PaymentRoute> {
        let graph = self.manual_graph.lock().unwrap();
        
        // If empty graph, add some mock data to show route finding
        if graph.edges.is_empty() {
            drop(graph);
            self.add_mock_graph_data();
            return self.find_route_dijkstra(source, destination, amount_msat, max_cltv_expiry);
        }
        
        self.find_route_dijkstra(source, destination, amount_msat, max_cltv_expiry)
    }
    
    /// Find a route using Dijkstra's algorithm
    fn find_route_dijkstra(
        &self,
        source: &str,
        destination: &str,
        amount_msat: u64,
        _max_cltv_expiry: u32,
    ) -> LightningResult<PaymentRoute> {
        let graph = self.manual_graph.lock().unwrap();
        
        // Check if source and destination are in the graph
        if !graph.edges.contains_key(source) {
            return Err(LightningError::PaymentError(
                format!("Source node not found in graph: {}", source)
            ));
        }
        
        if destination != source && !graph.edges.values().any(|edges| {
            edges.iter().any(|(target, _, _, _, _)| target == destination)
        }) {
            return Err(LightningError::PaymentError(
                format!("Destination node not found in graph: {}", destination)
            ));
        }
        
        // Special case for self-payment
        if source == destination {
            return Ok(PaymentRoute {
                hops: Vec::new(),
                total_amount_msat: amount_msat,
                total_fee_msat: 0,
                total_cltv_expiry_delta: 0,
            });
        }
        
        // Dijkstra's algorithm for finding lowest fee path
        let mut queue = BinaryHeap::new();
        let mut distances: HashMap<String, u64> = HashMap::new();
        let mut visited = HashSet::new();
        let mut prev_nodes: HashMap<String, (String, String)> = HashMap::new(); // (prev_node, channel_id)
        
        // Initialize with source node
        queue.push(NodeWithDistance {
            pubkey: source.to_string(),
            fee: 0,
            prev_node: None,
            prev_channel: None,
        });
        distances.insert(source.to_string(), 0);
        
        while let Some(node) = queue.pop() {
            // If we've reached the destination, reconstruct the path
            if node.pubkey == destination {
                return self.reconstruct_path(
                    &graph, 
                    source, 
                    destination, 
                    amount_msat, 
                    &prev_nodes
                );
            }
            
            // Skip if already visited
            if !visited.insert(node.pubkey.clone()) {
                continue;
            }
            
            // Check all outgoing edges
            if let Some(edges) = graph.edges.get(&node.pubkey) {
                for (target, channel_id, capacity, fee_base_msat, fee_proportional_millionths) in edges {
                    // Skip if insufficient capacity
                    if *capacity < amount_msat / 1000 {
                        continue;
                    }
                    
                    // Calculate fee for this hop
                    let fee = node.fee + (*fee_base_msat as u64) + 
                        (amount_msat * (*fee_proportional_millionths as u64) / 1_000_000);
                    
                    // Update if we found a better path
                    if !distances.contains_key(target) || fee < *distances.get(target).unwrap() {
                        distances.insert(target.clone(), fee);
                        prev_nodes.insert(
                            target.clone(), 
                            (node.pubkey.clone(), channel_id.clone())
                        );
                        
                        queue.push(NodeWithDistance {
                            pubkey: target.clone(),
                            fee,
                            prev_node: Some(node.pubkey.clone()),
                            prev_channel: Some(channel_id.clone()),
                        });
                    }
                }
            }
        }
        
        // If we get here, no path was found
        Err(LightningError::PaymentError(
            format!("No path found from {} to {}", source, destination)
        ))
    }
    
    /// Reconstruct a path from the pathfinding algorithm
    fn reconstruct_path(
        &self,
        graph: &Graph,
        source: &str,
        destination: &str,
        amount_msat: u64,
        prev_nodes: &HashMap<String, (String, String)>,
    ) -> LightningResult<PaymentRoute> {
        let mut current = destination.to_string();
        let mut path = Vec::new();
        let mut total_fee_msat = 0;
        
        // Start from destination and work backward
        while current != source {
            let (prev, channel_id) = prev_nodes.get(&current)
                .ok_or_else(|| LightningError::PaymentError(
                    format!("Path reconstruction failed, missing node: {}", current)
                ))?;
            
            // Get channel details
            let (src, dest, capacity, fee_base_msat, fee_proportional_millionths) = 
                graph.channels.get(channel_id)
                .ok_or_else(|| LightningError::PaymentError(
                    format!("Channel not found during path reconstruction: {}", channel_id)
                ))?;
            
            // Calculate fee for this hop
            let fee_msat = (*fee_base_msat as u64) + 
                (amount_msat * (*fee_proportional_millionths as u64) / 1_000_000);
            
            total_fee_msat += fee_msat;
            
            // Add hop to path (in reverse order)
            path.push(PaymentHop {
                src_node_id: src.clone(),
                dest_node_id: dest.clone(),
                channel_id: channel_id.clone(),
                amount_msat,
                fee_msat,
                cltv_expiry_delta: 40, // Default CLTV delta
            });
            
            current = prev.clone();
        }
        
        // Reverse the path to get source -> destination
        path.reverse();
        
        Ok(PaymentRoute {
            hops: path,
            total_amount_msat: amount_msat,
            total_fee_msat,
            total_cltv_expiry_delta: 40 * path.len() as u32, // 40 blocks per hop
        })
    }
    
    /// Add a channel to the router's graph
    pub fn add_channel(
        &self,
        channel_id: &str,
        node1: &str,
        node2: &str,
        capacity: u64,
        fee_base_msat: u32,
        fee_proportional_millionths: u32,
    ) -> LightningResult<()> {
        let mut graph = self.manual_graph.lock().unwrap();
        
        // Add channel info
        graph.channels.insert(
            channel_id.to_string(), 
            (
                node1.to_string(), 
                node2.to_string(), 
                capacity, 
                fee_base_msat, 
                fee_proportional_millionths
            )
        );
        
        // Add edges in both directions
        graph.edges.entry(node1.to_string())
            .or_insert_with(Vec::new)
            .push((
                node2.to_string(), 
                channel_id.to_string(), 
                capacity, 
                fee_base_msat, 
                fee_proportional_millionths
            ));
        
        graph.edges.entry(node2.to_string())
            .or_insert_with(Vec::new)
            .push((
                node1.to_string(), 
                channel_id.to_string(), 
                capacity, 
                fee_base_msat, 
                fee_proportional_millionths
            ));
        
        Ok(())
    }
    
    /// Remove a channel from the graph
    pub fn remove_channel(&self, channel_id: &str) -> LightningResult<()> {
        let mut graph = self.manual_graph.lock().unwrap();
        
        // Get channel info
        let channel_info = graph.channels.remove(channel_id)
            .ok_or_else(|| LightningError::ChannelError(
                format!("Channel not found: {}", channel_id)
            ))?;
        
        let (node1, node2, _, _, _) = channel_info;
        
        // Remove edges in both directions
        if let Some(edges) = graph.edges.get_mut(&node1) {
            edges.retain(|(target, id, _, _, _)| !(target == &node2 && id == channel_id));
        }
        
        if let Some(edges) = graph.edges.get_mut(&node2) {
            edges.retain(|(target, id, _, _, _)| !(target == &node1 && id == channel_id));
        }
        
        Ok(())
    }
    
    /// Update channel capacity
    pub fn update_channel_capacity(
        &self, 
        channel_id: &str, 
        new_capacity: u64
    ) -> LightningResult<()> {
        let mut graph = self.manual_graph.lock().unwrap();
        
        // Update channel info
        if let Some((node1, node2, _, fee_base_msat, fee_proportional_millionths)) = 
            graph.channels.get(channel_id) {
            
            // Create updated channel info
            let updated_info = (
                node1.clone(), 
                node2.clone(), 
                new_capacity, 
                *fee_base_msat, 
                *fee_proportional_millionths
            );
            
            // Update the channel info
            graph.channels.insert(channel_id.to_string(), updated_info);
            
            // Update edges in both directions
            if let Some(edges) = graph.edges.get_mut(node1) {
                for edge in edges.iter_mut() {
                    if edge.0 == *node2 && edge.1 == channel_id {
                        edge.2 = new_capacity;
                    }
                }
            }
            
            if let Some(edges) = graph.edges.get_mut(node2) {
                for edge in edges.iter_mut() {
                    if edge.0 == *node1 && edge.1 == channel_id {
                        edge.2 = new_capacity;
                    }
                }
            }
            
            Ok(())
        } else {
            Err(LightningError::ChannelError(
                format!("Channel not found: {}", channel_id)
            ))
        }
    }
    
    /// Add mock data to the graph for testing
    fn add_mock_graph_data(&self) {
        // Create a small test network with 5 nodes
        let mut graph = self.manual_graph.lock().unwrap();
        
        // Clear existing data
        graph.edges.clear();
        graph.channels.clear();
        
        // Generate node IDs
        let node_ids = [
            "02eadbd9e7557375161df8b646776a547c5097cc8288021e9ee72cb33327f912cd", // Our node
            "03f25d220b14f3daae528bbb98cf142caf3477c8d5258d9f81b0af0370163f0df2",
            "027a0d65b1ae0abad97fb80723d80c760b9e9c1f7a92fffb18ca3d57401225b56c",
            "023c6e150630c0a9bba412795203fa7ad86c9b24b103d8e05f0905d4b0f5bf6c3b",
            "035566252e83e2a30ec88140ea7948d505615f057b0e4c186a854cfbef365ea3c5"
        ];
        
        // Create channels between nodes
        let channels = [
            // channel_id, node1, node2, capacity, fee_base_msat, fee_proportional_millionths
            ("c1", node_ids[0], node_ids[1], 1_000_000, 1000, 1000),
            ("c2", node_ids[1], node_ids[2], 2_000_000, 1500, 500),
            ("c3", node_ids[2], node_ids[4], 1_500_000, 2000, 750),
            ("c4", node_ids[0], node_ids[3], 3_000_000, 1000, 100),
            ("c5", node_ids[3], node_ids[4], 2_500_000, 1200, 200),
            ("c6", node_ids[1], node_ids[3], 1_800_000, 800, 50),
        ];
        
        // Add channels to the graph
        for (channel_id, node1, node2, capacity, fee_base_msat, fee_proportional_millionths) in channels {
            // Add channel info
            graph.channels.insert(
                channel_id.to_string(), 
                (
                    node1.to_string(), 
                    node2.to_string(), 
                    capacity, 
                    fee_base_msat, 
                    fee_proportional_millionths
                )
            );
            
            // Add edges in both directions
            graph.edges.entry(node1.to_string())
                .or_insert_with(Vec::new)
                .push((
                    node2.to_string(), 
                    channel_id.to_string(), 
                    capacity, 
                    fee_base_msat, 
                    fee_proportional_millionths
                ));
            
            graph.edges.entry(node2.to_string())
                .or_insert_with(Vec::new)
                .push((
                    node1.to_string(), 
                    channel_id.to_string(), 
                    capacity, 
                    fee_base_msat, 
                    fee_proportional_millionths
                ));
        }
    }
} 