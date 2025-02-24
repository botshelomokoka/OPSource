from datetime import datetime, timedelta

class MetricAnalyzer:
    def optimize_model(self, model, metrics):
        window = self._create_time_window(metrics, hours=24)
        anomalies = self.detect_anomalies(window)
        return model.adjust_weights(anomalies)
    
    def _create_time_window(self, metrics, hours):
        return [m for m in metrics 
                if m.timestamp > datetime.now() - timedelta(hours=hours)] 