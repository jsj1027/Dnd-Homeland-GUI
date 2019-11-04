pub struct HealthComponent(AtomicUsize);

impl HealthComponent {
    fn new(initial: usize) -> HealthComponent {
        HealthComponent(AtomicUsize::new(initial))
    }

    fn get_health(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }

    fn subtract(&self, value: usize) -> usize {
        let current = self.0.load(Ordering::SeqCst);
        let new = if current < value { 0 } else { current - value };
        self.0.store(new, Ordering::SeqCst);
        new
    }

    fn heal(&self, value: usize) -> usize {
        let original = self.0.fetch_add(value, Ordering::SeqCst);
        original + value
    }
}
